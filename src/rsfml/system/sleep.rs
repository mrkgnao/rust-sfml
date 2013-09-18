/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Make thread sleeping
*/

use traits::wrappable::Wrappable;
use system::time::*;

#[doc(hidden)]
pub mod ffi {
    
    use system::time::*;

    extern "C" {
        pub fn sfSleep(duration : ffi::sfTime) -> ();
    }
}

/**
* Make the current thread sleep for a given duration
*/
#[fixed_stack_segment] #[inline(never)]
pub fn sleep(time : Time) -> () {
    unsafe {
        ffi::sfSleep(time.unwrap())
    }
}