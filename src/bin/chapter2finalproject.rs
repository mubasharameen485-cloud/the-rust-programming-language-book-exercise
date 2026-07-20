const FREEZING_POINT_F:f64=32.0;
fn analyze_tem(celsius:f64)->(f64,char){
    cal=((celsius * 1.8) + 32.0);
if cal > FREEZING_POINT_F{
    print!("F");
}
else if  cal<80.0 {
    print!("H")
}else {
    print!("N")
};
(f,char)

}


fn main(){
    let temps_celsius:  =[0.0, 40.0, 25.0, -10.0, 15.0];
    'system_loop:loop{
        print!("Analyzing weather data...");
        for temp in temps_celsius{
           analyze_temp(temp);
           let (f_val, status) = analyze_temp(temp);


    if status=='F'{
        print!("X degrees F is Freezing! ❄️");
        }else if status == 'H {
            print!("X degrees F is Hot! 🔥");
        }else if status == 'N{
            print!("X degrees F is Normal Weather. ⛅");
        
        }
    }
}
