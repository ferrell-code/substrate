// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Storage map type. Implements StorageDoubleMap, StorageIterableDoubleMap,
//! StoragePrefixedDoubleMap traits and their methods directly.

use codec::{FullCodec, Decode, EncodeLike, Encode};
use crate::{
	storage::{
		StorageAppend, StorageDecodeLength,
		types::{OptionQuery, QueryKindTrait, OnEmptyGetter},
	},
	traits::{GetDefault, StorageInstance},
};
use frame_metadata::{DefaultByteGetter, StorageEntryModifier};
use sp_std::vec::Vec;

/// A type that allow to store values for `(key1, key2)` couple. Similar to `StorageMap` but allow
/// to iterate and remove value associated to first key.
///
/// Each value is stored at:
/// ```nocompile
/// Twox128(Prefix::pallet_prefix())
///		++ Twox128(Prefix::STORAGE_PREFIX)
///		++ Hasher1(encode(key1))
///		++ Hasher2(encode(key2))
/// ```
///
/// # Warning
///
/// If the key1s (or key2s) are not trusted (e.g. can be set by a user), a cryptographic `hasher`
/// such as `blake2_128_concat` must be used for Hasher1 (resp. Hasher2). Otherwise, other values
/// in storage can be compromised.
pub struct StorageTripleMap<
	Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind=OptionQuery, OnEmpty=GetDefault
>(
	core::marker::PhantomData<(Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty)>
);

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
	crate::storage::generator::StorageTripleMap<Key1, Key2, Key3, Value> for
	StorageTripleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
    Hasher2: crate::hash::StorageHasher,
    Hasher3: crate::hash::StorageHasher,
	Key1: FullCodec,
    Key2: FullCodec,
    Key3: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: crate::traits::Get<QueryKind::Query> + 'static
{
	type Query = QueryKind::Query;
	type Hasher1 = Hasher1;
    type Hasher2 = Hasher2;
    type Hasher3 = Hasher3;
	fn module_prefix() -> &'static [u8] {
		Prefix::pallet_prefix().as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		QueryKind::from_optional_value_to_query(v)
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		QueryKind::from_query_to_optional_value(v)
	}
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
	crate::storage::StoragePrefixedMap<Value> for
	StorageTripleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
    Hasher2: crate::hash::StorageHasher,
    Hasher3: crate::hash::StorageHasher,
	Key1: FullCodec,
    Key2: FullCodec,
    Key3: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: crate::traits::Get<QueryKind::Query> + 'static
{
	fn module_prefix() -> &'static [u8] {
		<Self as crate::storage::generator::StorageTripleMap<Key1, Key2, Key3, Value>>::module_prefix()
	}
	fn storage_prefix() -> &'static [u8] {
		<Self as crate::storage::generator::StorageTripleMap<Key1, Key2, Key3, Value>>::storage_prefix()
	}
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
	StorageTripleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
    Hasher2: crate::hash::StorageHasher,
    Hasher3: crate::hash::StorageHasher,
	Key1: FullCodec,
    Key2: FullCodec,
    Key3: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: crate::traits::Get<QueryKind::Query> + 'static
{
	/// Get the storage key used to fetch a value corresponding to a specific key.
	pub fn hashed_key_for<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3) -> Vec<u8>
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::hashed_key_for(k1, k2, k3)
	}

	/// Does the value (explicitly) exist in storage?
	pub fn contains_key<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3) -> bool
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::contains_key(k1, k2, k3)
	}

	/// Load the value associated with the given key from the double map.
	pub fn get<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3) -> QueryKind::Query
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::get(k1, k2, k3)
	}

	/// Try to get the value for the given key from the double map.
	///
	/// Returns `Ok` if it exists, `Err` if not.
	pub fn try_get<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3) -> Result<Value, ()>
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
        
    {
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::try_get(k1, k2, k3)
	}

	/// Take a value from storage, removing it afterwards.
	pub fn take<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3) -> QueryKind::Query
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::take(k1, k2, k3)
	}

	/// Swap the values of two key-pairs.
    pub fn swap<XKArg1, XKArg2, XKArg3, 
                YKArg1, YKArg2, YKArg3>
    (x_k1: XKArg1, x_k2: XKArg2, x_k3:XKArg3, y_k1: YKArg1, y_k2: YKArg2, y_k3: YKArg3)
	where
		XKArg1: EncodeLike<Key1>,
        XKArg2: EncodeLike<Key2>,
        XKArg3: EncodeLike<Key3>,
		YKArg1: EncodeLike<Key1>,
        YKArg2: EncodeLike<Key2>,
        YKArg3: EncodeLike<Key3>,        
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::swap(x_k1, x_k2, x_k3, y_k1, y_k2, y_k3)
	}

	/// Store a value to be associated with the given keys from the double map.
	pub fn insert<KArg1, KArg2, KArg3, VArg>(k1: KArg1, k2: KArg2, k3: KArg3, val: VArg)
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		VArg: EncodeLike<Value>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::insert(k1, k2, k3, val)
	}

	/// Remove the value under the given keys.
	pub fn remove<KArg1, KArg2, KArg3>(k1: KArg1, k2: KArg2, k3: KArg3)
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::remove(k1, k2, k3)
	}

	/// Remove all values under the first key.
	pub fn remove_prefix<KArg1>(k1: KArg1) where KArg1: ?Sized + EncodeLike<Key1> {
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::remove_prefix(k1)
	}

	/// Iterate over values that share the first key.
	pub fn iter_prefix_values<KArg1>(k1: KArg1) -> crate::storage::PrefixIterator<Value>
	where KArg1: ?Sized + EncodeLike<Key1>
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::iter_prefix_values(k1)
	}

	/// Mutate the value under the given keys.
	pub fn mutate<KArg1, KArg2, KArg3, R, F>(k1: KArg1, k2: KArg2, k3: KArg3, f: F) -> R
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		F: FnOnce(&mut QueryKind::Query) -> R,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::mutate(k1, k2, k3, f)
	}

	/// Mutate the value under the given keys when the closure returns `Ok`.
	pub fn try_mutate<KArg1, KArg2, KArg3, R, E, F>(k1: KArg1, k2: KArg2, k3: KArg3, f: F) -> Result<R, E>
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		F: FnOnce(&mut QueryKind::Query) -> Result<R, E>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::try_mutate(k1, k2, k3, f)
	}

	/// Mutate the value under the given keys. Deletes the item if mutated to a `None`.
	pub fn mutate_exists<KArg1, KArg2, KArg3, R, F>(k1: KArg1, k2: KArg2, k3: KArg3, f: F) -> R
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		F: FnOnce(&mut Option<Value>) -> R,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::mutate_exists(k1, k2, k3, f)
	}

	/// Mutate the item, only if an `Ok` value is returned. Deletes the item if mutated to a `None`.
	pub fn try_mutate_exists<KArg1, KArg2, KArg3, R, E, F>(k1: KArg1, k2: KArg2, k3: KArg3, f: F) -> Result<R, E>
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		F: FnOnce(&mut Option<Value>) -> Result<R, E>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::try_mutate_exists(k1, k2, k3, f)
	}

	/// Append the given item to the value in the storage.
	///
	/// `Value` is required to implement [`StorageAppend`].
	///
	/// # Warning
	///
	/// If the storage item is not encoded properly, the storage will be overwritten
	/// and set to `[item]`. Any default value set for the storage item will be ignored
	/// on overwrite.
	pub fn append<Item, EncodeLikeItem, KArg1, KArg2, KArg3>(
		k1: KArg1,
        k2: KArg2,
        k3: KArg3,
		item: EncodeLikeItem,
	) where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		Value: StorageAppend<Item>,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::append(k1, k2, k3, item)
	}

	/// Read the length of the storage value without decoding the entire value under the
	/// given `key1` and `key2`.
	///
	/// `Value` is required to implement [`StorageDecodeLength`].
	///
	/// If the value does not exists or it fails to decode the length, `None` is returned.
	/// Otherwise `Some(len)` is returned.
	///
	/// # Warning
	///
	/// `None` does not mean that `get()` does not return a value. The default value is completly
	/// ignored by this function.
	pub fn decode_len<KArg1, KArg2, KArg3>(key1: KArg1, key2: KArg2, key3: KArg3) -> Option<usize>
	where
		KArg1: EncodeLike<Key1>,
        KArg2: EncodeLike<Key2>,
        KArg3: EncodeLike<Key3>,
		Value: StorageDecodeLength,
	{
		<Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>>::decode_len(key1, key2, key3)
	}

	/// Migrate an item with the given `key1` and `key2` from defunct `OldHasher1` and
	/// `OldHasher2` to the current hashers.
	///
	/// If the key doesn't exist, then it's a no-op. If it does, then it returns its value.
	pub fn migrate_keys<
		OldHasher1: crate::StorageHasher,
        OldHasher2: crate::StorageHasher,
        OldHasher3: crate::StorageHasher,
		KeyArg1: EncodeLike<Key1>,
        KeyArg2: EncodeLike<Key2>,
        KeyArg3: EncodeLike<Key3>,
	>(key1: KeyArg1, key2: KeyArg2, key3: KeyArg3) -> Option<Value> {
		<
			Self as crate::storage::StorageTripleMap<Key1, Key2, Key3, Value>
		>::migrate_keys::<OldHasher1, OldHasher2, OldHasher3, _, _, _>(key1, key2, key3)
	}

	/// Remove all value of the storage.
	pub fn remove_all() {
		<Self as crate::storage::StoragePrefixedMap<Value>>::remove_all()
	}

	/// Iter over all value of the storage.
	///
	/// NOTE: If a value failed to decode becaues storage is corrupted then it is skipped.
	pub fn iter_values() -> crate::storage::PrefixIterator<Value> {
		<Self as crate::storage::StoragePrefixedMap<Value>>::iter_values()
	}

	/// Translate the values of all elements by a function `f`, in the map in no particular order.
	/// By returning `None` from `f` for an element, you'll remove it from the map.
	///
	/// NOTE: If a value fail to decode because storage is corrupted then it is skipped.
	///
	/// # Warning
	///
	/// This function must be used with care, before being updated the storage still contains the
	/// old type, thus other calls (such as `get`) will fail at decoding it.
	///
	/// # Usage
	///
	/// This would typically be called inside the module implementation of on_runtime_upgrade.
	pub fn translate_values<OldValue: Decode, F: FnMut(OldValue) -> Option<Value>>(f: F) {
		<Self as crate::storage::StoragePrefixedMap<Value>>::translate_values(f)
	}
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
	StorageTripleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher + crate::ReversibleStorageHasher,
    Hasher2: crate::hash::StorageHasher + crate::ReversibleStorageHasher,
    Hasher3: crate::hash::StorageHasher + crate::ReversibleStorageHasher,
	Key1: FullCodec,
    Key2: FullCodec,
    Key3: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: crate::traits::Get<QueryKind::Query> + 'static
{
	/// Enumerate all elements in the map with first key `k1` in no particular order.
	///
	/// If you add or remove values whose first key is `k1` to the map while doing this, you'll get
	/// undefined results.
	pub fn iter_prefix(k1: impl EncodeLike<Key1>) -> crate::storage::PrefixIterator<(Key3, Value)> {
		<Self as crate::storage::IterableStorageTripleMap<Key1, Key2, Key3, Value>>::iter_prefix(k1)
	}

	/// Remove all elements from the map with first key `k1` and iterate through them in no
	/// particular order.
	///
	/// If you add elements with first key `k1` to the map while doing this, you'll get undefined
	/// results.
	pub fn drain_prefix(k1: impl EncodeLike<Key1>) -> crate::storage::PrefixIterator<(Key3, Value)> {
		<Self as crate::storage::IterableStorageTripleMap<Key1, Key2, Key3, Value>>::drain_prefix(k1)
	}

	/// Enumerate all elements in the map in no particular order.
	///
	/// If you add or remove values to the map while doing this, you'll get undefined results.
	pub fn iter() -> crate::storage::PrefixIterator<(Key1, Key2, Key3, Value)> {
		<Self as crate::storage::IterableStorageTripleMap<Key1, Key2, Key3, Value>>::iter()
	}

	/// Remove all elements from the map and iterate through them in no particular order.
	///
	/// If you add elements to the map while doing this, you'll get undefined results.
	pub fn drain() -> crate::storage::PrefixIterator<(Key1, Key2, Key3, Value)> {
		<Self as crate::storage::IterableStorageTripleMap<Key1, Key2, Key3, Value>>::drain()
	}

	/// Translate the values of all elements by a function `f`, in the map in no particular order.
	///
	/// By returning `None` from `f` for an element, you'll remove it from the map.
	///
	/// NOTE: If a value fail to decode because storage is corrupted then it is skipped.
	pub fn translate<O: Decode, F: FnMut(Key1, Key2, Key3, O) -> Option<Value>>(f: F) {
		<Self as crate::storage::IterableStorageTripleMap<Key1, Key2, Key3, Value>>::translate(f)
	}
}

/// Part of storage metadata for a storage double map.
///
/// NOTE: Generic hashers is supported.
pub trait StorageTripleMapMetadata {
	const MODIFIER: StorageEntryModifier;
	const NAME: &'static str;
	const DEFAULT: DefaultByteGetter;
	const HASHER1: frame_metadata::StorageHasher;
    const HASHER2: frame_metadata::StorageHasher;
    const HASHER3: frame_metadata::StorageHasher;
}

impl<Prefix, Hasher1, Hasher2, Hasher3, Key1, Key2, Key3, Value, QueryKind, OnEmpty> StorageTripleMapMetadata
	for StorageTripleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Hasher3, Key3, Value, QueryKind, OnEmpty> where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
    Hasher2: crate::hash::StorageHasher,
    Hasher3: crate::hash::StorageHasher,
	Key1: FullCodec,
    Key2: FullCodec,
    Key3: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: crate::traits::Get<QueryKind::Query> + 'static
{
	const MODIFIER: StorageEntryModifier = QueryKind::METADATA;
	const HASHER1: frame_metadata::StorageHasher = Hasher1::METADATA;
    const HASHER2: frame_metadata::StorageHasher = Hasher2::METADATA;
    const HASHER3: frame_metadata::StorageHasher = Hasher3::METADATA;
	const NAME: &'static str = Prefix::STORAGE_PREFIX;
	const DEFAULT: DefaultByteGetter =
		DefaultByteGetter(&OnEmptyGetter::<QueryKind::Query, OnEmpty>(core::marker::PhantomData));
}

#[cfg(test)]
mod test {
	use super::*;
	use sp_io::{TestExternalities, hashing::twox_128};
	use crate::hash::*;
	use crate::storage::types::ValueQuery;
	use frame_metadata::StorageEntryModifier;

	struct Prefix;
	impl StorageInstance for Prefix {
		fn pallet_prefix() -> &'static str { "test" }
		const STORAGE_PREFIX: &'static str = "foo";
	}

	struct ADefault;
	impl crate::traits::Get<u32> for ADefault {
		fn get() -> u32 {
			97
		}
	}

	#[test]
	fn test() {
		type A = StorageTripleMap<
			Prefix, Blake2_128Concat, u16, Twox64Concat, u8, Twox64Concat, u8, u32, OptionQuery
		>;
		type AValueQueryWithAnOnEmpty = StorageTripleMap<
			Prefix, Blake2_128Concat, u16, Twox64Concat, u8, Twox64Concat, u8, u32, ValueQuery, ADefault
		>;
		type B = StorageTripleMap<Prefix, Blake2_256, u16, Twox128, u8, Twox128, u8, u32, ValueQuery>;
		type C = StorageTripleMap<Prefix, Blake2_128Concat, u16, Twox64Concat, u8, Twox64Concat, u8, u8, ValueQuery>;
		type WithLen = StorageTripleMap<Prefix, Blake2_128Concat, u16, Twox64Concat, u8, Twox64Concat, u8, Vec<u32>>;

		TestExternalities::default().execute_with(|| {
			let mut k: Vec<u8> = vec![];
			k.extend(&twox_128(b"test"));
			k.extend(&twox_128(b"foo"));
			k.extend(&3u16.blake2_128_concat());
            k.extend(&30u8.twox_64_concat());
            k.extend(&40u8.twox_64_concat());
			assert_eq!(A::hashed_key_for(3, 30, 40).to_vec(), k);

			assert_eq!(A::contains_key(3, 30, 40), false);
			assert_eq!(A::get(3, 30, 40), None);
			assert_eq!(AValueQueryWithAnOnEmpty::get(3, 30, 40), 97);

			A::insert(3, 30, 40, 10);
			assert_eq!(A::contains_key(3, 30, 40), true);
			assert_eq!(A::get(3, 30, 40), Some(10));
			assert_eq!(AValueQueryWithAnOnEmpty::get(3, 30, 40), 10);

			A::swap(3, 30, 40, 2, 20, 22);
			assert_eq!(A::contains_key(3, 30, 40), false);
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(3, 30, 40), None);
			assert_eq!(AValueQueryWithAnOnEmpty::get(3, 30, 40), 97);
			assert_eq!(A::get(2, 20, 22), Some(10));
			assert_eq!(AValueQueryWithAnOnEmpty::get(2, 20, 22), 10);

			A::remove(2, 20, 22);
			assert_eq!(A::contains_key(2, 20, 22), false);
			assert_eq!(A::get(2, 20, 22), None);

			AValueQueryWithAnOnEmpty::mutate(2, 20, 22, |v| *v = *v * 2);
			AValueQueryWithAnOnEmpty::mutate(2, 20, 22, |v| *v = *v * 2);
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(97 * 4));

			A::remove(2, 20, 22);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, 22, |v| {
				*v = *v * 2; Ok(())
			});
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, 22, |v| {
				*v = *v * 2; Ok(())
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(97 * 4));

			A::remove(2, 20, 22);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, 22, |v| {
				*v = *v * 2; Err(())
			});
			assert_eq!(A::contains_key(2, 20, 22), false);

			A::remove(2, 20, 22);
			AValueQueryWithAnOnEmpty::mutate_exists(2, 20, 22, |v| {
				assert!(v.is_none());
				*v = Some(10);
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(10));
			AValueQueryWithAnOnEmpty::mutate_exists(2, 20, 22, |v| {
				*v = Some(v.unwrap() * 10);
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(100));

			A::remove(2, 20, 22);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, 22, |v| {
				assert!(v.is_none());
				*v = Some(10);
				Ok(())
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(10));
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, 22, |v| {
				*v = Some(v.unwrap() * 10);
				Ok(())
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(100));
			assert_eq!(A::try_get(2, 20, 22), Ok(100));
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, 22, |v| {
				*v = Some(v.unwrap() * 10);
				Err(())
			});
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(100));

			A::insert(2, 20, 22, 10);
			assert_eq!(A::take(2, 20, 22), Some(10));
			assert_eq!(A::contains_key(2, 20, 22), false);
			assert_eq!(AValueQueryWithAnOnEmpty::take(2, 20, 22), 97);
			assert_eq!(A::contains_key(2, 20, 22), false);
			assert_eq!(A::try_get(2, 20, 22), Err(()));

			B::insert(2, 20, 22, 10);
			assert_eq!(A::migrate_keys::<Blake2_256, Twox128, Twox128, _,  _, _>(2, 20, 22), Some(10));
			assert_eq!(A::contains_key(2, 20, 22), true);
			assert_eq!(A::get(2, 20, 22), Some(10));

			A::insert(3, 30, 60, 10);
			A::insert(4, 40, 80, 10);
			A::remove_all();
			assert_eq!(A::contains_key(3, 30, 60), false);
			assert_eq!(A::contains_key(4, 40, 80), false);

			A::insert(3, 30, 60, 10);
			A::insert(4, 40, 80, 10);
			assert_eq!(A::iter_values().collect::<Vec<_>>(), vec![10, 10]);

			C::insert(3, 30, 60, 10);
			C::insert(4, 40, 80, 10);
			A::translate_values::<u8,_>(|v| Some((v * 2).into()));
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 80, 20), (3, 30, 60, 20)]);

			A::insert(3, 30, 60, 10);
			A::insert(4, 40, 80, 10);
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 80, 10), (3, 30, 60, 10)]);
			assert_eq!(A::drain().collect::<Vec<_>>(), vec![(4, 40, 80, 10), (3, 30, 60, 10)]);
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![]);

			C::insert(3, 30, 2, 10);
			C::insert(4, 40, 2, 10);
			A::translate::<u8,_>(|k1, k2, k3, v| Some((k1 * k2 as u16 * k3 as u16 * v as u16).into()));
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 2, 3200), (3, 30, 2, 1800)]);

			assert_eq!(A::MODIFIER, StorageEntryModifier::Optional);
			assert_eq!(AValueQueryWithAnOnEmpty::MODIFIER, StorageEntryModifier::Default);
			assert_eq!(A::HASHER1, frame_metadata::StorageHasher::Blake2_128Concat);
			assert_eq!(A::HASHER2, frame_metadata::StorageHasher::Twox64Concat);
			assert_eq!(A::HASHER3, frame_metadata::StorageHasher::Twox64Concat);
			assert_eq!(
				AValueQueryWithAnOnEmpty::HASHER1,
				frame_metadata::StorageHasher::Blake2_128Concat
			);
			assert_eq!(
				AValueQueryWithAnOnEmpty::HASHER2,
				frame_metadata::StorageHasher::Twox64Concat
			);
			assert_eq!(
				AValueQueryWithAnOnEmpty::HASHER3,
				frame_metadata::StorageHasher::Twox64Concat
			);
			assert_eq!(A::NAME, "foo");
			assert_eq!(AValueQueryWithAnOnEmpty::DEFAULT.0.default_byte(), 97u32.encode());
			assert_eq!(A::DEFAULT.0.default_byte(), Option::<u32>::None.encode());
			
			WithLen::remove_all();
			assert_eq!(WithLen::decode_len(3, 30, 33), None);
			WithLen::append(0, 50, 100, 10);
			assert_eq!(WithLen::decode_len(0, 50, 100), Some(1));

			A::insert(3, 30, 33, 11);
			A::insert(3, 31, 34, 12);
			A::insert(4, 40, 44, 13);
			A::insert(4, 41, 45, 14);
			assert_eq!(A::iter_prefix_values(3).collect::<Vec<_>>(), vec![12, 11]);
			assert_eq!(A::iter_prefix(3).collect::<Vec<_>>(), vec![(31, 12), (30, 11)]);
			assert_eq!(A::iter_prefix_values(4).collect::<Vec<_>>(), vec![13, 14]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);
/*
			A::remove_prefix(3);
			assert_eq!(A::iter_prefix(3).collect::<Vec<_>>(), vec![]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);

			assert_eq!(A::drain_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![]);
			assert_eq!(A::drain_prefix(4).collect::<Vec<_>>(), vec![]);A::swap(3, 30, 2, 20);
			assert_eq!(A::contains_key(3, 30), false);
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(3, 30), None);
			assert_eq!(AValueQueryWithAnOnEmpty::get(3, 30), 97);
			assert_eq!(A::get(2, 20), Some(10));
			assert_eq!(AValueQueryWithAnOnEmpty::get(2, 20), 10);

			A::remove(2, 20);
			assert_eq!(A::contains_key(2, 20), false);
			assert_eq!(A::get(2, 20), None);

			AValueQueryWithAnOnEmpty::mutate(2, 20, |v| *v = *v * 2);
			AValueQueryWithAnOnEmpty::mutate(2, 20, |v| *v = *v * 2);
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(97 * 4));

			A::remove(2, 20);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, |v| {
				*v = *v * 2; Ok(())
			});
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, |v| {
				*v = *v * 2; Ok(())
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(97 * 4));

			A::remove(2, 20);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate(2, 20, |v| {
				*v = *v * 2; Err(())
			});
			assert_eq!(A::contains_key(2, 20), false);

			A::remove(2, 20);
			AValueQueryWithAnOnEmpty::mutate_exists(2, 20, |v| {
				assert!(v.is_none());
				*v = Some(10);
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(10));
			AValueQueryWithAnOnEmpty::mutate_exists(2, 20, |v| {
				*v = Some(v.unwrap() * 10);
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(100));

			A::remove(2, 20);
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, |v| {
				assert!(v.is_none());
				*v = Some(10);
				Ok(())
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(10));
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, |v| {
				*v = Some(v.unwrap() * 10);
				Ok(())
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(100));
			assert_eq!(A::try_get(2, 20), Ok(100));
			let _: Result<(), ()> = AValueQueryWithAnOnEmpty::try_mutate_exists(2, 20, |v| {
				*v = Some(v.unwrap() * 10);
				Err(())
			});
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(100));


			A::insert(2, 20, 10);
			assert_eq!(A::take(2, 20), Some(10));
			assert_eq!(A::contains_key(2, 20), false);
			assert_eq!(AValueQueryWithAnOnEmpty::take(2, 20), 97);
			assert_eq!(A::contains_key(2, 20), false);
			assert_eq!(A::try_get(2, 20), Err(()));

			B::insert(2, 20, 10);
			assert_eq!(A::migrate_keys::<Blake2_256, Twox128, _, _>(2, 20), Some(10));
			assert_eq!(A::contains_key(2, 20), true);
			assert_eq!(A::get(2, 20), Some(10));

			A::insert(3, 30, 10);
			A::insert(4, 40, 10);
			A::remove_all();
			assert_eq!(A::contains_key(3, 30), false);
			assert_eq!(A::contains_key(4, 40), false);

			A::insert(3, 30, 10);
			A::insert(4, 40, 10);
			assert_eq!(A::iter_values().collect::<Vec<_>>(), vec![10, 10]);

			C::insert(3, 30, 10);
			C::insert(4, 40, 10);
			A::translate_values::<u8,_>(|v| Some((v * 2).into()));
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 20), (3, 30, 20)]);

			A::insert(3, 30, 10);
			A::insert(4, 40, 10);
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 10), (3, 30, 10)]);
			assert_eq!(A::drain().collect::<Vec<_>>(), vec![(4, 40, 10), (3, 30, 10)]);
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![]);

			C::insert(3, 30, 10);
			C::insert(4, 40, 10);
			A::translate::<u8,_>(|k1, k2, v| Some((k1 * k2 as u16 * v as u16).into()));
			assert_eq!(A::iter().collect::<Vec<_>>(), vec![(4, 40, 1600), (3, 30, 900)]);

			assert_eq!(A::MODIFIER, StorageEntryModifier::Optional);
			assert_eq!(AValueQueryWithAnOnEmpty::MODIFIER, StorageEntryModifier::Default);
			assert_eq!(A::HASHER1, frame_metadata::StorageHasher::Blake2_128Concat);
			assert_eq!(A::HASHER2, frame_metadata::StorageHasher::Twox64Concat);
			assert_eq!(
				AValueQueryWithAnOnEmpty::HASHER1,
				frame_metadata::StorageHasher::Blake2_128Concat
			);
			assert_eq!(
				AValueQueryWithAnOnEmpty::HASHER2,
				frame_metadata::StorageHasher::Twox64Concat
			);
			assert_eq!(A::NAME, "foo");
			assert_eq!(AValueQueryWithAnOnEmpty::DEFAULT.0.default_byte(), 97u32.encode());
			assert_eq!(A::DEFAULT.0.default_byte(), Option::<u32>::None.encode());

			WithLen::remove_all();
			assert_eq!(WithLen::decode_len(3, 30), None);
			WithLen::append(0, 100, 10);
			assert_eq!(WithLen::decode_len(0, 100), Some(1));

			A::insert(3, 30, 11);
			A::insert(3, 31, 12);
			A::insert(4, 40, 13);
			A::insert(4, 41, 14);
			assert_eq!(A::iter_prefix_values(3).collect::<Vec<_>>(), vec![12, 11]);
			assert_eq!(A::iter_prefix(3).collect::<Vec<_>>(), vec![(31, 12), (30, 11)]);
			assert_eq!(A::iter_prefix_values(4).collect::<Vec<_>>(), vec![13, 14]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);

			A::remove_prefix(3);
			assert_eq!(A::iter_prefix(3).collect::<Vec<_>>(), vec![]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);

			assert_eq!(A::drain_prefix(4).collect::<Vec<_>>(), vec![(40, 13), (41, 14)]);
			assert_eq!(A::iter_prefix(4).collect::<Vec<_>>(), vec![]);
			assert_eq!(A::drain_prefix(4).collect::<Vec<_>>(), vec![]);
*/
		})
	}
}