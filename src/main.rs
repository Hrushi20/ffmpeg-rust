use std::{mem::MaybeUninit, num::ParseIntError, process::exit, vec, time::Duration, fs::{self, File}, io::Write, fmt::format };

mod generated;


static mut image_number:u32 = 0;

fn main() {
    unsafe {
        let mut avFormatContext = MaybeUninit::<u32>::uninit();
        generated::avformatAllocContext(avFormatContext.as_mut_ptr() as u32);

        let file = "assets/small_bunny_1080p_60fps.mp4";
        
        let res =generated::avformatOpenInput(avFormatContext.as_mut_ptr() as u32, file.as_ptr(), file.len());
        println!("Res: {}",res);

        let no_of_streams = generated::avformatFindStreamInfo(avFormatContext.as_mut_ptr() as u32);
        println!("Res: {}",no_of_streams);


        let mut avCodec:MaybeUninit<u32> = MaybeUninit::<u32>::uninit(); 
        let mut avCodecParameters:MaybeUninit<u32> = MaybeUninit::<u32>::uninit();
        let mut video_stream_index:i32 = -1 ;

        for i in 0..no_of_streams {
            let mut pLocalCodecParameters= MaybeUninit::<u32>::uninit();

            generated::avcodecParameters(avFormatContext.as_mut_ptr() as u32, i, pLocalCodecParameters.as_mut_ptr() as u32);

            let mut pLocalCodec= MaybeUninit::<u32>::uninit();

            match generated::avcodecFindDecoder(pLocalCodecParameters.as_mut_ptr() as u32, pLocalCodec.as_mut_ptr() as u32)  {
                0 => {
                    // Only taking video for transformation.
                    video_stream_index = i as i32;
                    avCodec = pLocalCodec;
                    pLocalCodecParameters = pLocalCodecParameters;
                },
                1 => {
                    println!("This is audio ");
                },
                _=> println!("Inavaid type"),
            };

            if video_stream_index == -1 {
                println!("File doesn't contain a video stream");
            }

        }

        let pCodecContext = MaybeUninit::<u32>::uninit();
        let res = generated::avcodecAllocContext3(avCodec.as_ptr() as u32, pCodecContext.as_ptr() as u32);

        let res = generated::avcodecParametersToContext(pCodecContext.as_ptr() as u32, avCodecParameters.as_ptr() as u32);

        generated::avcodecOpen2(avCodec.as_ptr() as u32, pCodecContext.as_ptr() as u32);


        let frame = MaybeUninit::<u32>::uninit();
        generated::avFrameAlloc(frame.as_ptr() as u32);


        let packet = MaybeUninit::<u32>::uninit();
        let res = generated::avPacketAlloc(packet.as_ptr() as u32);

        let mut response = 0;
        let mut how_many_packets_to_process = 8;

        while generated::avReadFrame(frame.as_ptr() as u32, packet.as_ptr() as u32) >= 0 {
            if generated::getStreamIndex(packet.as_ptr() as u32) as i32  == video_stream_index {
                response = decode(packet.as_ptr(),pCodecContext.as_ptr(),frame.as_ptr());

                if response == 100 {break};
                if --how_many_packets_to_process <=0 {break};
            }

            generated::avPacketUnref(packet.as_ptr() as u32);

        }
    };
}

unsafe fn  decode(packet:*const u32,pCodecContext:*const u32,frame:*const u32)->u32{
    let mut response = generated::avcodecSendPacket(packet as u32, pCodecContext as u32);
    if response == 100 {return response};

    while response >= 0 {
        response = generated::avcodecReceiveFrame(frame as u32, pCodecContext as u32);

        if response == 20 {
            break;
        }else if response == 100 {
            println!("Error while receiving Frame");
            return response;
        }
        if response >= 0 {
            let filename = format!("assets/frame-{}.pgm",image_number);
            image_number+=1;

            // Remove snprintf and write to file. Done... Also check for yellow thing or RGB.

            // generated::snprintf(buf.as_ptr(), buf.len(), pCodecContext as u32);
            // Write data to file

            let mut dimensions:Vec<u32> = Vec::new();
            let mut bufLen = MaybeUninit::<u32>::uninit();

            // generated::frameData(frame as u32, frameData.as_ptr() as u32, frameLen.as_ptr() as u32);

            dimensions.resize(3, 0);
            generated::frameDimensions(frame as u32, dimensions.as_mut_ptr() as *mut u8, bufLen.as_mut_ptr() as usize);

            let bufLen = std::ptr::read(bufLen.as_ptr());

            let mut frame_buf:Vec<u8> = Vec::new();
            frame_buf.resize((dimensions[1] * dimensions[2]) as usize, 0);

            generated::frameData(frame as u32, frame_buf.as_mut_ptr() as *mut u8, frame_buf.len());

            saveGrayscaleFrame(&filename,frame_buf,dimensions);
        }
    }
    // std::thread::sleep(Duration::from_millis(1000));
    return 0;
}

unsafe fn saveGrayscaleFrame(filename:&String,frame_buf:Vec<u8>, dimensions:Vec<u32>){
    let mut f = File::create(filename).unwrap();

    let header = format!("P5\n{} {}\n{}\n",dimensions[1],dimensions[2],255);
    f.write_all(header.as_bytes());
    
    f.write_all(&frame_buf);

}