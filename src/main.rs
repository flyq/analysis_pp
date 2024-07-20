use ark_serialize::CanonicalSerialize;
use nexus_core::prover::nova::{pp, types::SeqPP};

fn main() {
    let pp: SeqPP = pp::load_pp("./nexus-public-nova-seq-16.zst").unwrap();

    println!("ro_config size: {}", get_size(&pp.ro_config));
    println!("shape size: {}", get_size(&pp.shape));
    println!("shape_secondary size: {}", get_size(&pp.shape_secondary));
    println!("pp size: {}", get_size(&pp.pp));
    println!("pp_secondary size: {}", get_size(&pp.pp_secondary));
    println!("digest size: {}", get_size(&pp.digest));

    println!("shape: {}", pp.shape);
    println!(
        "shape SparseMatrix A size\ndata: {}, indices: {}, indptr: {}",
        pp.shape.A.data.len(),
        pp.shape.A.indices.len(),
        pp.shape.A.indptr.len()
    );
    println!(
        "shape SparseMatrix B\ndata: {}, indices: {}, indptr: {}",
        pp.shape.B.data.len(),
        pp.shape.B.indices.len(),
        pp.shape.B.indptr.len()
    );
    println!(
        "shape SparseMatrix C\ndata: {}, indices: {}, indptr: {}",
        pp.shape.C.data.len(),
        pp.shape.C.indices.len(),
        pp.shape.C.indptr.len()
    );
}

fn get_size<T: CanonicalSerialize>(t: &T) -> usize {
    let mut buffer = Vec::new();
    t.serialize_compressed(&mut buffer).unwrap();
    buffer.len()
}

/*
ro_config size: 7128
shape size: 457055984
shape_secondary size: 10881160
pp size: 16288968
pp_secondary size: 96776
digest size: 32
shape: R1CSShape { num_constraints: 509030, num_vars: 490374, num_io: 2, A: [_, 3841133], B: [_, 6804135], C: [_, 475710] }
shape SparseMatrix A size
data: 3841133, indices: 3841133, indptr: 509031
shape SparseMatrix B
data: 6804135, indices: 6804135, indptr: 509031
shape SparseMatrix C
data: 475710, indices: 475710, indptr: 509031
*/
