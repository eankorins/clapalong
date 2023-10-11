use clap::Parser;

#[derive(Debug, Parser)]
struct Claps {
    ///Message to display
    message: String,
}
// <epg_import version="6" xmlns="http://barrowa.com/common/epg/format/barrowa/v6">
//   <schedule display_name="Test - Animal Planet HD" id="Test-AniPlaHD" xmlns="">
//     <event end_time="20180501 19:30:00" start_time="20180501 18:30:00">
//       <description extended_synopsis="A wide range of products can be bought at attractive prices from the comfort of one's home." language="eng" short_synopsis="" title="Teleshopping"/>
//       <content nibble1="3" nibble2="0"/>
//       <parental_rating country="GBR" rating="6"/>
//     </event>
//     <event end_time="20180501 20:00:00" start_time="20180501 19:30:00">


fn main() {
    let args = Claps::parse();
    let message_len = args.message.len();
    let half_len = args.message.len() / 2;
    print_text(message_len , "_");
    print_text_spacing(half_len, &args.message);
    print_text(message_len, "-");
    print_text_spacing(half_len, "/     \\");
    print_text_spacing(half_len, "\\     /");
    print_text_spacing(half_len, ">o_o<") 
}
fn print_text(num: usize, text: &str){
    for _ in 0..num{
        print!("{}", text);
    }
    println!();
}
fn print_text_spacing(num: usize, text: &str){
    for idx in 0..num {
        print!(" ")
    }
    println!("{}", text);
}

