

pub mod voice_algorithms {

    const PI: f32 = std::f32::consts::PI;

    #[inline]
    pub fn sine_function(xval: f32) -> f32 {
        return xval.sin();
    }

    #[inline]
    pub fn piano_function(xval: f32) -> f32 {
        // let mut tmp: f32 = (xval * PI).sin().powf(3.0);
        // // print!("({}^2,{}),", angle_constant, tmp);
        // tmp += (PI * (xval + 0.666666)).sin();
        return
            ((-0.25)*((3.0*PI*xval).sin())) +
            (0.25 * ((PI*xval).sin())) +
            ((((3.0 as f32).powf(0.2))/2.0)*((PI*xval).cos()));
    }

    #[inline]
    pub fn piano_function2(xval: f32) -> f32 {
        // return
        //     ((5.0*(xval/2.0/PI)).sin()+
        //     (5.0*(xval/PI)).cos())/2.0
        // return
        //     (xval.sin()+
        //     (2.0*xval).sin())/3.0
        // return
        //     (xval.sin()+
        //     ((2.0*xval)/2.0).sin()+
        //     ((3.0*xval)/3.0).sin()+
        //     ((4.0*xval)/3.0).sin())/1.6
        // return
        // return
        //     (xval*(xval.log(2.0))).sin();
        return
            xval.sin().powf(2.0)*
            (xval+0.25).sin()*
            (xval+0.5).sin()*
            (xval+0.75).sin()
        //     (xval.sin()+
        //     ((2.0*xval)*0.11).sin()+
        //     ((3.0*xval)*0.35).sin()+
        //     ((4.0*xval)*0.06).sin()+
        //     ((5.0*xval)*0.05).sin()+
        //     ((6.0*xval)*0.045).sin())/1.6
        // return
        //     (xval.sin()+
        //     ((2.0*xval)*0.4).sin()+
        //     ((3.0*xval)*0.2).sin()+
        //     ((4.0*xval)*0.3).sin())/1.6
        // return
        //     xval.sin()+
        //     2.0*
        // return (
        //     (0.2*xval.sin())+
        //     (0.1*xval.sin()*2.0)+
        //     (0.035*xval.sin()*3.0)+
        //     (0.025*xval.sin()*4.0)+
        //     (0.022*xval.sin()*5.0)+
        //     (0.019*xval.sin()*6.0)
        // )*3.5
    }

    // #[inline]
    // pub fn piano_

    #[inline]
    pub fn square_funtion(xval: f32) -> f32 {
        if xval.sin() > 0.0 {
            return 1.0
        }else {
            return -1.0
        }
    }

    #[inline]
    pub fn cos_tan_function(xval: f32) -> f32 {
        return xval.tan().cos();
    }

    #[inline]
    pub fn double_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin())/1.8
    }

    #[inline]
    pub fn triple_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin())/2.5
    }

    #[inline]
    pub fn quadruple_sine(xval: f32) -> f32 {
        return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin()+(xval*4.0).sin())/3.3
    }
}