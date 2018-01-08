#![feature(duration_extras)]
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

// impl Service for HelloWorld {
//     // boilerplate hooking up hyper's server types
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     // The future representing the eventual Response your call will
//     // resolve to. This can change to whatever Future you need.
//     type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

//     fn call(&self, _req: Request) -> Self::Future {
//         // We're currently ignoring the Request
//         // And returning an 'ok' Future, which means it's ready
//         // immediately, and build a Response with the 'PHRASE' body.
//         Box::new(futures::future::ok(
//             Response::new()
//                 .with_header(ContentLength(PHRASE.len() as u64))
//                 .with_body(PHRASE)
//         ))
//     }
// }

// fn main() {
//     let mut core = Core::new().unwrap();
//     let client = Client::new(&core.handle());
//     let uri = "http://httpbin.org/ip".parse().unwrap();
//     let work = client.get(uri).and_then(|res| {
//         println!("Response: {}", res.status());

//         res.body().concat2().and_then(move |body| {
//             let v: Value = serde_json::from_slice(&body).map_err(|e| {
//                 io::Error::new(
//                     io::ErrorKind::Other,
//                     e
//                 )
//             })?;
//             println!("current IP address is {}", v["origin"]);
//             Ok(())
//         })
//     });
//     core.run(work).unwrap();
// }

#[cfg(windows)]
extern crate winapi;
use std::io::Error;
use std::mem::size_of;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HMODULE; 
use winapi::shared::minwindef::FALSE;
use winapi::um::psapi::EnumProcesses;

#[cfg(windows)]
fn print_message() -> Result<i32, Error> {
    use std::time::Instant;
    use std::io::Error;
    use std::mem;
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
    use winapi::um::winnt::PROCESS_VM_READ;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::shared::minwindef::TRUE;
    use winapi::um::psapi::EnumProcessModules;
    use winapi::um::psapi::GetModuleBaseNameW;
    let mut xxx: [DWORD; 1024] = [0 as DWORD; 1024];
    let mut bytes_ret: DWORD = 0;
    let now = Instant::now();
    let size = size_of::<[DWORD; 1024]>() as u32;
    unsafe {
        EnumProcesses(xxx.as_mut_ptr(), size, &mut bytes_ret);
    }

    let num_processes = bytes_ret /( size_of::<DWORD>() as u32);
    let pids = &xxx[0..num_processes as usize];

    for pid in pids {
        use winapi::shared::minwindef::MAX_PATH;
        let mut proc_name: [u16; MAX_PATH] = [0; MAX_PATH];
        unsafe{ 
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                                    FALSE, *pid );
            let mut h_mod: HMODULE = 0 as HMODULE; 
            let mut cb_needed: DWORD = 0 as DWORD;
            let tmp = EnumProcessModules( handle,  &mut h_mod, size_of::<HMODULE>() as u32, &mut cb_needed); 
            if  tmp == TRUE
            {
                GetModuleBaseNameW( handle, h_mod, proc_name.as_mut_ptr(), 
                                (mem::size_of_val(&proc_name)/mem::size_of::<char>()) as u32);
            }
        }
        println!("proc_name: {}", String::from_utf16_lossy(&proc_name));
        println!("pid: {}", pid);
    }

    let dur = now.elapsed();
    println!(
        "Time elasped {} secs + {} millis ",
        dur.as_secs(),
        dur.subsec_millis()
    ); 
    println!("Found {} processes", num_processes);
    println!("first pid = {}", xxx[0]);
    println!("last pid = {}", xxx[num_processes as usize - 1]);
    println!("last pid + 1 = {}", xxx[num_processes as usize]);

    // println!("{}", ret);
    if 1 == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(1)
    }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

fn main() {
    print_message().unwrap();
}