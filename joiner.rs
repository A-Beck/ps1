use std::rand::random;
use std::str;
use std::os;
use std::io::File;
use std::io;

fn main(){
	let args: ~[~str] = os::args();

    if args.len() != 3 {
        println("please list your two files"); 
    } 

    else {
    	let file1 = &args[1];
    	let file2 = &args[2];
    	let path1 = Path::new(file1.clone());
    	let path2 = Path::new(file2.clone());
    	let msg1_file = File::open(&path1);
    	let msg2_file = File::open(&path2);

    	match (msg1_file, msg2_file) {
			(Some(mut msg1), Some(mut msg2)) => {
				let msg_bytes1: ~[u8] = msg1.read_to_end();
				let msg_bytes2: ~[u8] = msg2.read_to_end();

				let decoded = xor(msg_bytes1,msg_bytes2);
				let s = str::from_utf8(decoded);
				println(s);

			}

			(_,_) => {fail!("bad files");}	
    	}
	}

}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}