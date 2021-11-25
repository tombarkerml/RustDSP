fn main() {
    println!("Hello, world!");

    let b = (0.29287490, 0.58574979, 0.29287490);
    let a = (-7.17336609e-17, 0.17149959);
    let mut filt1 = Biquad::new(b, a);

    let values = vec![0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0];

    for sample in values{
        let out = filt1.process_sample(sample);
        println!("value {}", out)
    }


}


pub struct Biquad{
    /* https://ccrma.stanford.edu/~jos/fp/Direct_Form_II.html */
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    vn: f64,
    vn_1: f64, //previous input sample
    vn_2: f64, //previous+1 input sample
    yn: f64,
}

impl Biquad {

    fn new(b: (f64, f64, f64), a: (f64, f64))->Biquad {
        Biquad{b0: b.0, b1: b.1, b2: b.2, a1: a.0, a2: a.1, vn: 0.0, vn_1: 0.0, vn_2: 0.0, yn:0.0}
    }


    fn update(&mut self, xn: f64){
        //pass a new value into the filter. vn_2 takes value of vn_1
        self.vn_1=self.vn;
        self.vn_2=self.vn_1;        
        self.vn = xn-self.a1*self.vn_1-self.a2*self.vn_2;
        self.yn = self.b0*self.vn + self.b1*self.vn_1+self.b2*self.vn_2;
        

    }


    fn process_sample(&mut self, input: f64)->f64{
        self.update(input);
        self.yn
    }

}