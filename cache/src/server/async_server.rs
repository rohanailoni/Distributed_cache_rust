use std::ffi::c_void;
use std::net::{IpAddr, Ipv4Addr};
use std::ptr;
use std::str::FromStr;
use inet_aton::inet_aton;
use libc::{accept, sockaddr};
use log::{info};



pub(crate) unsafe fn run_async_tcp_server(){
    println!("starting the asynchronous service");

    const MAX_CLIENTS:usize=100;

    let mut events:[libc::epoll_event; MAX_CLIENTS] = [libc::epoll_event{events:0,u64:0}; MAX_CLIENTS];
    //let mut events:libc::epoll_event=Default::default();


    //syscall socker required domain name
    // /
    // a The domain argument specifies a communication domain; this
    //        selects the protocol family which will be used for communication.nd required type like what kind of socker required
    //AF_INET==Ipv4 internet protocol like TCP/IP model
    // The socket has the indicated type, which specifies the
    //   communication semantics.

    //ref:-https://docs.rs/libc/latest/libc/fn.socket.html, https://man7.org/linux/man-pages/man2/socket.2.html
    let serverFD=libc::socket(libc::AF_INET,libc::SOCK_STREAM|libc::O_NONBLOCK,0);
    if serverFD==-1 {
        //that means that error occured here!!!
        println!("failed to create a socker returing the process");
        return;
    }
    println!("successful created socket");

    match set_nonblock(serverFD.clone(), true) {
        Ok(()) => {
            println!("setting non blocking successful");
        }
        Err(err) => {
            println!("Setting nonblocking failed with error: {} (check the doc for the error)", err);
        }
    }


    const IP_ADDRESS: &[u8] = "127.0.0.1".as_bytes();
    let opt=inet_aton(IP_ADDRESS);
    let  s_addr: u32 =match opt {
        Some(x)=>x,
        None=>1
    };
    const PORT: u16 = 7878;

    let mut socker_address:libc::sockaddr_in=libc::sockaddr_in{
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: PORT.to_be(),
        sin_addr: libc::in_addr {s_addr:s_addr },
        sin_zero: [0;8],
    };
    let mut address_size=std::mem::size_of_val(&socker_address) as libc::socklen_t;
    // when a socker is created it is just created in name space but no address assigned to it
    // return value:-On success, zero is returned. On error, -1 is returned, and errno is set appropriately.
    //ref for error Values:-https://linux.die.net/man/2/bind,https://docs.rs/libc/latest/libc/fn.bind.html
    let err=libc::bind(serverFD,
               &socker_address as *const _ as *const libc::sockaddr,
               address_size,);
    if err!=0 {
        println!("binding Failed with error expression {err} check the doc for the errror");
        return;
    }
    println!("Successfully binding the port");

    //now we have binded we have to listen to the connections
    //return :-On success, zero is returned.  On error, -1 is returned, and
    //        errno is set to indicate the error.
    //ref:-https://docs.rs/libc/latest/libc/fn.listen.html, https://man7.org/linux/man-pages/man2/listen.2.html

    let err=libc::listen(serverFD, MAX_CLIENTS as libc::c_int);
    if err!=0 {
        println!("Listen Failed with error expression {err} check the doc for the error");
        return;

    }
    println!("we are listening to the connection");

    //this is a sample of accepting the connection
    // let clientFD= libc::accept(serverFD, &mut socker_address as *mut _ as *mut libc::sockaddr, &mut address_size);
    //
    // if clientFD<0{
    //     println!("Accept Failed with error expression {err} check the doc for the error");
    //     return;
    // }
    // let mut buffer = [0; 1024];
    // let read_bytes=libc::read(clientFD, buffer.as_mut_ptr() as *mut libc::c_void,buffer.len());
    // println!("{:?}",String::from_utf8_lossy(&buffer));

    // end of accepting the connection





    // now to use the power of async we use Epoll first we create a Epoll instance and we add file descriptor we want to monitor
    //ref:-https://man7.org/linux/man-pages/man7/epoll.7.html,

    let epoll_fd=libc::epoll_create1(0); //if flag is 0 is acts as epoll_create;
    if epoll_fd<0{
        println!("epoll creating instance Failed with error expression {epoll_fd} check the doc for the error");
        return;
    }
    let mut first_epoll_event: libc::epoll_event=libc::epoll_event{
        events: libc::EPOLLIN as u32,
        u64: serverFD as u64,
    };
    let err=libc::epoll_ctl(epoll_fd,libc::EPOLL_CTL_ADD,serverFD,&mut first_epoll_event);
    if err<0{
        return;
    }
    println!("The first socket sucessfully added to the epoll kernal instance");
    loop{
        println!("epoll waiting with event");
        let event=libc::epoll_wait(epoll_fd,&mut events[0],MAX_CLIENTS as libc::c_int,-1);
        let mut first_even =false;
        if event>0{
            for i in 0..event{
                let x=events[i as usize];
                if x.events!=0 && x.u64!=0{
                    println!("The server file descriptor is {} and the epoll event has event as {} and {} and length events detected {}",serverFD, { x.events }, { x.u64 },event);
                }
                //that means we are directly pinging the socket itself
                if x.u64== serverFD as u64{
                    let clientFD= libc::accept(serverFD, &mut socker_address as *mut _ as *mut libc::sockaddr, &mut address_size);

                    if clientFD<0{
                        println!("Accept Failed with error expression {err} check the doc for the error");
                        return;
                    }
                    match set_nonblock(clientFD,true){
                        Ok(()) => {
                            println!("setting non blocking for clientsuccessful");
                        }
                        Err(err) => {
                            println!("Setting nonblocking for client failed with error: {} (check the doc for the error)", err);
                        }
                    }
                    let mut first_client_event =libc::epoll_event{
                        events: libc::EPOLLIN as u32,
                        u64: clientFD as u64,
                    };
                    let err=libc::epoll_ctl(epoll_fd,libc::EPOLL_CTL_ADD,clientFD,&mut first_client_event);
                    if err<0{
                        println!("failed to add client to epoll instance failed with error {err}");
                        return;
                    }
                    println!("succcessfull added clientFd to epoll instance");

                }else{
                    let mut buffer = [0; 1024];
                    let read_bytes=libc::read(x.u64 as libc::c_int, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len());
                    libc::write(x.u64 as libc::c_int, buffer.as_ptr() as *const libc::c_void, read_bytes as libc::size_t);
                    println!("reading the bytes from descriptor {} and the buffer is {:?}",{x.u64},String::from_utf8_lossy(&buffer));
                    libc::close(x.u64 as libc::c_int);
                    if !first_even{

                        first_even=true;
                        break;
                    }
                }
            }
            if first_even{
                //break;
            }
        }
        println!("Ending the loop");
    }

    libc::close(epoll_fd);
    libc::close(serverFD);
}



unsafe fn set_nonblock(fd:libc::c_int,nonblocking:bool)->Result<(), libc::c_int>{
    //ref;-https://man7.org/linux/man-pages/man2/fcntl.2.html;
    let mut flag =libc::fcntl(fd, libc::F_GETFL, 0);
    if flag==-1{
        println!("fectching flag status  Failed with error expression {flag} check the doc for the error");
        return Err(-1);
    }
    if nonblocking{
        flag=flag | libc::O_NONBLOCK;
    }else{
        flag &=!libc::O_NONBLOCK;
    }
    flag=libc::fcntl(fd,libc::F_SETFL,flag);
    if flag==-1{
        println!("setting nonblocking Failed with error expression {flag} check the doc for the error");
        Err(-1)
    }else{
        Ok(())
    }

}