

pub mod voice_algorithms {

    use std::f32::consts::PI;

    #[inline]
    pub fn _sine_function(xval: f32) -> f32 {
        return xval.sin();
    }

    #[inline]
    pub fn _electric_piano1(xval: f32) -> f32 {
        return
            ((5.0*(xval/2.0/PI)).sin()+
            (5.0*(xval/PI)).cos())/2.0
    }

    #[inline]
    pub fn _electric_piano2(xval: f32) -> f32 {
        return
            (xval.sin()+
            (2.0*xval).sin())/3.0
    }

    #[inline]
    pub fn _vibrawave1(xval: f32) -> f32 {
        // vibra wave
        return
            (xval.sin()+
            ((2.0*xval)*0.11).sin()+
            ((3.0*xval)*0.35).sin()+
            ((4.0*xval)*0.06).sin()+
            ((5.0*xval)*0.05).sin()+
            ((6.0*xval)*0.045).sin())/1.6
    }

    #[inline]
    pub fn _vibrawave2(xval: f32) -> f32 {
        // vibra wave 2
        return
            (xval.sin()+
            ((2.0*xval)*0.4).sin()+
            ((3.0*xval)*0.2).sin()+
            ((4.0*xval)*0.3).sin())/1.6
    }

    #[inline]
    pub fn _square_wave1(xval: f32) -> f32 {
        if xval.sin() > 0.0 {
            return 1.0
        }else {
            return -1.0
        }
    }

    // #[inline]
    // pub fn _square_wave2(xval: f32) -> f32 {
    //     // println
    //     return (xval.sin().powf(0.333)+xval.sin())/10000000.0; // good square sound
    // }

    #[inline]
    pub fn _double_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin())/1.8
    }

    #[inline]
    pub fn _triple_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin())/2.5
    }

    #[inline]
    pub fn _quadruple_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin()+(xval*4.0).sin())/3.3
    }

    // #[inline]
    // pub fn _test(xval: f32) -> f32 {
    //     // return ((xval.sin()).powf(0.333)).sin()
    //     // return (xval.sin()*xval).powf(0.2).sin()
    //     // return (xval.sin()*xval.powf(2.0)).powf(0.2).sin()
    //     // return xval.sin().powf(0.333);
    //     // return (xval.powf(2.0).sin())/2.0+(xval+0.5).cos()
    //     // return ((xval.powf(2.0).sin()/10.0)+xval.sin())/1.2 // good space sound
    // }

    #[inline]
    pub fn _clarinet1(xval: f32) -> f32 {
        return (
            xval.sin()+
            ((2.0*xval).sin()*0.04)+
            ((3.0*xval).sin()*0.99)+
            ((4.0*xval).sin()*0.12)+
            ((5.0*xval).sin()*0.53)+
            ((6.0*xval).sin()*0.11)+
            ((7.0*xval).sin()*0.26)+
            ((8.0*xval).sin()*0.05)+
            ((9.0*xval).sin()*0.24)+
            ((10.0*xval).sin()*0.07)+
            ((11.0*xval).sin()*0.02)+
            ((12.0*xval).sin()*0.03)+
            ((13.0*xval).sin()*0.02)+
            ((14.0*xval).sin()*0.03)
        )/2.0
    }

    // #[inline]
    // pub fn test(xval: f32) -> f32 {
    //     return xval.log10().sin()
    // }

}