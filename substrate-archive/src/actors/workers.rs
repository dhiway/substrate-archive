// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

pub mod blocks;
pub mod database;
pub mod extrinsics_decoder;
mod metadata;
pub mod storage_aggregator;

pub use self::database::DatabaseActor;
pub use self::metadata::MetadataActor;
pub use blocks::BlocksIndexer;
pub use extrinsics_decoder::ExtrinsicsDecoder;
pub use storage_aggregator::StorageAggregator;
