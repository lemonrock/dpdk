// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Active : Default + Clone
{
	type T: Eq + Hash;
	
	const Maximum: usize;
	
	#[inline(always)]
	fn constructor(index: usize) -> Self::T;
	
	#[inline(always)]
	fn count(&self) -> usize;
	
	#[inline(always)]
	fn none() -> Self;
	
	#[inline(always)]
	fn all() -> Self;
	
	#[inline(always)]
	fn value(&self, index: usize) -> bool;
	
	#[inline(always)]
	fn set(&mut self, index: usize, toValue: bool);

	fn asLinuxString(&self) -> String
	{
		let mut values = String::with_capacity(1024);
		let mut afterFirst = false;
		self.iterateEnabled(|index|
		{
			if afterFirst
			{
				values.push(',');
			}
			else
			{
				afterFirst = true;
			}
			values.push_str(&format!("{}", index));
		});
		values.shrink_to_fit();
		values
	}
	
	fn parse_from_file_path(path: &Path) -> Result<Self, ListParseError>
	{
		let mut openFile = File::open(path)?;
		let mut rawString = String::with_capacity(256);
		let bytesRead = openFile.read_to_string(&mut rawString)?;
		
		// Remove trailing LineFeed
		if bytesRead == 0
		{
			return Err(ListParseError::EmptyFile);
		}
		
		let shouldBeLineFeed = rawString.remove(bytesRead - 1);
		if shouldBeLineFeed != '\n'
		{
			return Err(ListParseError::FileContentsDidNotEndWithATrailingLineFeed);
		}
		
		Self::parse(&rawString)
	}
	
	// eg "2,4-31,32-63"
	fn parse(linuxString: &str) -> Result<Self, ListParseError>
	{
		#[inline(always)]
	    fn parseIndex<A: Active>(indexString: &str, description: &'static str) -> Result<usize, ListParseError>
	    {
	        match indexString.parse()
	        {
	            Ok(index) => if index as usize >= A::Maximum
				{
					Err(ListParseError::IndexExceedsMaximum(description, index, A::Maximum))
				}
				else
				{
					Ok(index)
				},
	            Err(error) => Err(ListParseError::CouldNotParseIndex(description, indexString.to_owned(), error)),
	        }
	    }
		
		let mut result = Self::none();
		
		// Prevents mis-sorted strings
		let mut nextMinimumIndex = 0;
	    for indexOrRange in linuxString.split(',')
	    {
			if indexOrRange.is_empty()
			{
				return Err(ListParseError::ContainsAnEmptyIndexOrRange);
			}
        
	        let mut iterator = indexOrRange.splitn(2, '-');
			
	        let first = parseIndex::<Self>(iterator.next().unwrap(), "first")?;
			
			if first < nextMinimumIndex
			{
				return Err(ListParseError::ContainsMisSortedIndices(first, nextMinimumIndex));
			}
			
	        if let Some(second) = iterator.last()
	        {
	            let second = parseIndex::<Self>(second, "second")?;
				if first >= second
				{
					return Err(ListParseError::RangeIsNotAnAscendingRangeWithMoreThanOneElement(first, second));
				}
				
				for index in first..(second + 1)
				{
					result.enable(index);
				}
				nextMinimumIndex = second;
	        }
	        else
	        {
				let sole = first;
				result.enable(sole);
				nextMinimumIndex = sole;
	        }
	    }
		
		Ok
		(
			result
		)
	}

	fn asVec(&self) -> Vec<Self::T>
	{
		let mut vec = Vec::with_capacity(self.count());
		self.iterateEnabled(|index|
		{
			let value = Self::constructor(index);
			vec.push(value);
		});
		vec
	}

	fn asHashSet(&self) -> HashSet<Self::T>
	{
		let mut set = HashSet::with_capacity(self.count());
		self.iterateEnabled(|index|
		{
			let value = Self::constructor(index);
			set.insert(value);
		});
		set
	}
	
	fn intersect(&self, other: &Self) -> Self
	{
		let mut result = Self::none();
		
		for index in 0..Self::Maximum
		{
			let left = self.value(index);
			let right = other.value(index);
			result.set(index, left & right)
		}
		
		result
	}
	
	fn iterateEnabled<F>(&self, mut callIfEnabled: F)
	where F: FnMut(usize)
	{
		for index in 0..Self::Maximum
		{
			if self.isEnabled(index)
			{
				callIfEnabled(index);
			}
		}
	}
	
	fn iterateEnabledWithEarlyReturn<F, E>(&self, mut callIfEnabled: F) -> Result<(), E>
	where F: FnMut(usize) -> Result<(), E>
	{
		for index in 0..Self::Maximum
		{
			if self.isEnabled(index)
			{
				callIfEnabled(index)?;
			}
		}
		Ok(())
	}
	
	fn lowestEnabled(&self) -> Option<usize>
	{
		for index in 0..Self::Maximum
		{
			if self.isEnabled(index)
			{
				return Some(index);
			}
		}
		
		None
	}
	
	#[inline(always)]
	fn hasAtLeastOneActive(&self) -> bool
	{
		self.count() != 0
	}
	
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.count() == 0
	}
	
	#[inline(always)]
	fn isInvalid(index: usize) -> bool
	{
		index >= Self::Maximum
	}
	
	#[inline(always)]
	fn isEnabled(&self, index: usize) -> bool
	{
		self.value(index)
	}
	
	#[inline(always)]
	fn isDisabled(&self, index: usize) -> bool
	{
		!self.value(index)
	}
	
	#[inline(always)]
	fn enable(&mut self, index: usize)
	{
		self.set(index, true);
	}
	
	#[inline(always)]
	fn disable(&mut self, index: usize)
	{
		self.set(index, false);
	}
	
	#[inline(always)]
	fn enableAll(&mut self)
	{
		self.enableAllUpToExclusive(Self::Maximum)
	}
	
	#[inline(always)]
	fn enableAllUpToExclusive(&mut self, indexTo: usize)
	{
		debug_assert!(indexTo >= Self::Maximum, "index '{}' is not less than Maximum '{}'", indexTo, Self::Maximum);
		
		self.enableAllFromInclusiveUpToExclusive(0, indexTo)
	}
	
	#[inline(always)]
	fn enableAllFromInclusiveUpToExclusive(&mut self, indexFrom: usize, indexTo: usize)
	{
		debug_assert!(indexTo >= Self::Maximum, "indexTo '{}' is not less than Maximum '{}'", indexTo, Self::Maximum);
		debug_assert!(indexFrom < indexTo, "indexFrom '{}' is not less than indexTo '{}'", indexFrom, indexTo);
		
		for index in indexFrom..indexTo
		{
			self.enable(index);
		}
	}
	
	#[inline(always)]
	fn disableAll(&mut self)
	{
		self.disableAllUpToExclusive(Self::Maximum)
	}
	
	#[inline(always)]
	fn disableAllUpToExclusive(&mut self, indexTo: usize)
	{
		debug_assert!(indexTo >= Self::Maximum, "index '{}' is not less than Maximum '{}'", indexTo, Self::Maximum);
		
		self.disableAllFromInclusiveUpToExclusive(0, indexTo)
	}
	
	#[inline(always)]
	fn disableAllFromInclusiveUpToExclusive(&mut self, indexFrom: usize, indexTo: usize)
	{
		debug_assert!(indexTo >= Self::Maximum, "indexTo '{}' is not less than Maximum '{}'", indexTo, Self::Maximum);
		debug_assert!(indexFrom < indexTo, "indexFrom '{}' is not less than indexTo '{}'", indexFrom, indexTo);
		
		for index in indexFrom..indexTo
		{
			self.disable(index);
		}
	}
}
