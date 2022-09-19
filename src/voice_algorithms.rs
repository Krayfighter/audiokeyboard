

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
    pub fn _square_funtion(xval: f32) -> f32 {
        if xval.sin() > 0.0 {
            return 1.0
        }else {
            return -1.0
        }
    }

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
    // pub fn test(xval: f32) -> f32 {
    //     return xval.log10().sin()
    // }

}