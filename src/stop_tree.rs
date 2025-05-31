use std::io::Write;
use std::fs::File;
use std::num::NonZero;
use anyhow::Result;
use kiddo::immutable::float::kdtree::{AlignedArchivedImmutableKdTree, ImmutableKdTreeRK};
use kiddo::ImmutableKdTree;
use memmap::{Mmap, MmapOptions};
use rkyv::ser::Serializer;
use rkyv::ser::serializers::{AlignedSerializer, BufferScratch, CompositeSerializer};
use gtfs::Stop;

const BUFFER_LEN: usize = 300_000_000;
const SCRATCH_LEN: usize = 300_000_000;

const KDTREE_FILENAME: &str = "stops_kdtree.rkyv";
const VEC_FILENAME: &str = "stops_vec.rkyv";

pub type ArchivedTree<'a> = AlignedArchivedImmutableKdTree<'a, f64, u64, 2, 32>;

async fn download_feed(
    client: &reqwest::Client, agency: &str, url: &str
) -> Result<Vec<Stop>> {
    println!("Downloading {}", agency);
    let resp = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
        .send()
        .await?;
    let content = resp.bytes().await?;

    let reader = std::io::Cursor::new(content);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    println!("Done with {}", agency);
    let vec = gtfs::read_gtfs_objects_from_zip(&mut zip, agency)?.collect();
    vec
}

fn stops_to_kdtree(stops: &Vec<Stop>) -> ImmutableKdTree<f64, 2> {
    let coords: Vec<[f64; 2]> = stops.iter()
        .map(|stop| [stop.stop_lat, stop.stop_lon])
        .collect();
    ImmutableKdTree::new_from_slice(&coords)
}

pub async fn download_and_pack_feed(
    client: &reqwest::Client, agency: &str, url: &str
) -> Result<()> {
    let stops = download_feed(client, agency, url).await?;
    let tree = stops_to_kdtree(&stops);
    let tree_rk: ImmutableKdTreeRK<f64, u64, 2, 32> = tree.into();

    let mut serialize_buffer = rkyv::AlignedVec::with_capacity(BUFFER_LEN);
    let mut serialize_scratch = rkyv::AlignedVec::with_capacity(SCRATCH_LEN);
    unsafe {
        serialize_scratch.set_len(SCRATCH_LEN);
    }
    serialize_buffer.clear();
    let mut serializer = CompositeSerializer::new(
        AlignedSerializer::new(&mut serialize_buffer),
        BufferScratch::new(&mut serialize_scratch),
        rkyv::Infallible,
    );
    serializer
        .serialize_value(&tree_rk)
        .expect("Could not serialize with rkyv");

    let buf = serializer.into_serializer().into_inner();
    let mut file = File::create(KDTREE_FILENAME)?;
    file.write_all(buf)?;

    // let kdtree_bytes = rkyv::to_bytes::<_, 256>(&tree_rk).unwrap();
    let vec_bytes = rkyv::to_bytes::<_, 256>(&stops).unwrap();
    // let mut file = File::create(KDTREE_FILENAME)?;
    // file.write_all(&kdtree_bytes)?;
    let mut file = File::create(VEC_FILENAME)?;
    file.write_all(&vec_bytes)?;
    Ok(())
}

pub struct ArchivedStopTree<'a> {
    _kdtree_mmap: Mmap,
    _vec_mmap: Mmap,
    pub kdtree: ArchivedTree<'a>,
    pub vec: Vec<Stop>,
}

impl<'a> ArchivedStopTree<'a> {
    pub fn unpack_from_files() -> Result<Self> {
        let kdtree_mmap = unsafe { MmapOptions::new().map(&File::open(KDTREE_FILENAME)?)? };
        let buf = unsafe {
            // The following call to transmute is safe (I think) because kdtree_mmap will be moved
            // into the same struct as kdtree when this function returns, meaning both objects will
            // have the same lifetime. Since Mmap implements Unpin, kdtree's reference to kdtree_mmap
            // can be safely moved with it (I think this is what Unpin means?)
            std::mem::transmute::<&[u8], &'a [u8]>(kdtree_mmap.as_ref())
        };
        let kdtree: ArchivedTree<'a> = AlignedArchivedImmutableKdTree::from_bytes(buf);

        let vec_mmap = unsafe { MmapOptions::new().map(&File::open(VEC_FILENAME)?)? };
        let vec = unsafe {
            rkyv::from_bytes_unchecked::<Vec<Stop>>(&vec_mmap)
                .expect("Failed to deserialize Vec<Stop>")
        };

        Ok(Self {
            _kdtree_mmap: kdtree_mmap,
            _vec_mmap: vec_mmap,
            kdtree,
            vec,
        })
    }

    pub fn find_nearest(&self, lat: f64, lon: f64, limit: NonZero<usize>) -> Vec<&Stop> {
        let nearest_n: Vec<_> = self.kdtree.nearest_n::<kiddo::SquaredEuclidean>(&[lat,lon], limit);
        nearest_n.iter().map(|neighbour| {
            &self.vec[neighbour.item as usize]
        }).collect()
    }
}