use crate::errors::Result;
use crate::fsshttpb::data_element::storage_index::StorageIndex;
use crate::fsshttpb::data_element::storage_manifest::StorageManifest;
use crate::fsshttpb::packaging::Packaging;
use crate::onestore::header::StoreHeader;
use crate::onestore::object_space::{ObjectSpace, Revision};
use crate::types::cell_id::CellId;
use crate::types::guid::Guid;
use std::collections::{HashMap, HashSet};

pub(crate) mod header;
pub(crate) mod mapping_table;
pub(crate) mod object;
pub(crate) mod object_space;
mod revision_role;
pub(crate) mod types;

#[derive(Debug)]
pub(crate) struct OneStore<'a> {
    schema: Guid,
    header: StoreHeader,
    data_root: ObjectSpace<'a>,
    object_spaces: HashMap<CellId, ObjectSpace<'a>>,
}

impl<'a> OneStore<'a> {
    pub fn schema_guid(&self) -> Guid {
        self.schema
    }

    pub(crate) fn data_root(&'a self) -> &'a ObjectSpace {
        &self.data_root
    }

    pub(crate) fn object_space(&'a self, space_id: CellId) -> Option<&'a ObjectSpace<'a>> {
        self.object_spaces.get(&space_id)
    }
}

pub(crate) fn parse_store(package: &Packaging) -> Result<OneStore> {
    let mut parsed_object_spaces = HashSet::new();

    // [ONESTORE] 2.7.1: Parse storage manifest
    let storage_index = package.find_storage_index();
    let storage_manifest = package.find_storage_manifest();

    let header_cell_id = find_header_cell_id(storage_manifest);

    let header_cell_mapping_id = storage_index
        .find_cell_mapping_id(header_cell_id)
        .expect("header cell mapping id not found");

    // [ONESTORE] 2.7.2: Parse header cell
    let header_cell = package
        .data_element_package
        .find_objects(header_cell_mapping_id, &storage_index)
        .into_iter()
        .next()
        .expect("no header object in header cell");

    let header = StoreHeader::parse(header_cell);

    parsed_object_spaces.insert(header_cell_id);

    // FIXME: document revision cache
    let mut revision_cache = HashMap::new();

    // Parse data root

    let data_root_cell_id = find_data_root_cell_id(storage_manifest);
    let (_, data_root) = parse_object_space(
        data_root_cell_id,
        storage_index,
        &package,
        &mut revision_cache,
    );

    parsed_object_spaces.insert(data_root_cell_id);

    // Parse other object spaces

    let mut object_spaces = HashMap::new();

    for mapping in storage_index.cell_mappings.values() {
        if mapping.id.is_nil() {
            continue;
        }

        if parsed_object_spaces.contains(&mapping.cell_id) {
            continue;
        }

        let (id, group) = parse_object_space(
            mapping.cell_id,
            storage_index,
            &package,
            &mut revision_cache,
        );
        object_spaces.insert(id, group);
    }

    Ok(OneStore {
        schema: storage_manifest.id,
        header,
        data_root,
        object_spaces,
    })
}

fn parse_object_space<'a, 'b>(
    cell_id: CellId,
    storage_index: &'a StorageIndex,
    package: &'a Packaging,
    revision_cache: &'b mut HashMap<CellId, Revision<'a>>,
) -> (CellId, ObjectSpace<'a>) {
    let mapping = storage_index
        .cell_mappings
        .get(&cell_id)
        .expect("cell mapping not found");

    ObjectSpace::parse(mapping, storage_index, package, revision_cache)
}

fn find_header_cell_id(manifest: &StorageManifest) -> CellId {
    manifest
        .roots
        .get(&exguid!({{1A5A319C-C26B-41AA-B9C5-9BD8C44E07D4}, 1}))
        .copied()
        .expect("no header cell root")
}

fn find_data_root_cell_id(manifest: &StorageManifest) -> CellId {
    manifest
        .roots
        .get(&exguid!({{84DEFAB9-AAA3-4A0D-A3A8-520C77AC7073}, 2}))
        .copied()
        .expect("no header cell root")
}
