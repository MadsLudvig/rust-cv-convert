#![allow(unused_macros)]
#![allow(unused_imports)]
macro_rules ! if_nalgebra { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "nalgebra_0-26" , feature = "nalgebra_0-27" , feature = "nalgebra_0-28" , feature = "nalgebra_0-29" , feature = "nalgebra_0-30" , feature = "nalgebra_0-31" , feature = "nalgebra_0-32"))] $ item) * } ; }
pub(crate) use if_nalgebra;
macro_rules ! has_nalgebra { ($ ($ item : item) *) => { crate :: macros :: if_nalgebra ! { # [allow (unused_imports)] use crate :: "nalgebra" as _ ; $ ($ item) * } } }
pub(crate) use has_nalgebra;
macro_rules ! if_opencv { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "opencv_0-63" , feature = "opencv_0-64" , feature = "opencv_0-65" , feature = "opencv_0-66" , feature = "opencv_0-67" , feature = "opencv_0-68" , feature = "opencv_0-69" , feature = "opencv_0-70" , feature = "opencv_0-71" , feature = "opencv_0-72" , feature = "opencv_0-73" , feature = "opencv_0-74" , feature = "opencv_0-75" , feature = "opencv_0-76"))] $ item) * } ; }
pub(crate) use if_opencv;
macro_rules ! has_opencv { ($ ($ item : item) *) => { crate :: macros :: if_opencv ! { # [allow (unused_imports)] use crate :: "opencv" as _ ; $ ($ item) * } } }
pub(crate) use has_opencv;
macro_rules ! if_ndarray { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "ndarray_0-15"))] $ item) * } ; }
pub(crate) use if_ndarray;
macro_rules ! has_ndarray { ($ ($ item : item) *) => { crate :: macros :: if_ndarray ! { # [allow (unused_imports)] use crate :: "ndarray" as _ ; $ ($ item) * } } }
pub(crate) use has_ndarray;
macro_rules ! if_image { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "image_0-23" , feature = "image_0-24"))] $ item) * } ; }
pub(crate) use if_image;
macro_rules ! has_image { ($ ($ item : item) *) => { crate :: macros :: if_image ! { # [allow (unused_imports)] use crate :: "image" as _ ; $ ($ item) * } } }
pub(crate) use has_image;
macro_rules ! if_tch { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "tch_0-10"))] $ item) * } ; }
pub(crate) use if_tch;
macro_rules ! has_tch { ($ ($ item : item) *) => { crate :: macros :: if_tch ! { # [allow (unused_imports)] use crate :: "tch" as _ ; $ ($ item) * } } }
pub(crate) use has_tch;
macro_rules ! if_imageproc { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "imageproc_0-23"))] $ item) * } ; }
pub(crate) use if_imageproc;
macro_rules ! has_imageproc { ($ ($ item : item) *) => { crate :: macros :: if_imageproc ! { # [allow (unused_imports)] use crate :: "imageproc" as _ ; $ ($ item) * } } }
pub(crate) use has_imageproc;
