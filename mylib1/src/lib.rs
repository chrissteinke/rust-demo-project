use std::ffi::{CString, c_char};

///
///
///
#[derive(Debug)]
#[repr(C)]
pub struct DataT {
    flag: bool, // demonstrating memory layout
    val: isize,
}

// 
unsafe extern "C" {
    fn lookup_data(id: *const c_char) -> *const DataT;

    fn free_data(ptr: *const DataT);
}

#[derive(Debug)]
pub enum LibError {
    BadObjId,
    LookupFailed,
}

///
///
///
#[derive(Debug)]
pub enum DataWrapper {
    Native(DataT),
    Foreign(*const DataT),
}

///
///
///
impl DataWrapper {
    pub fn new(id: &str) -> Result<DataWrapper, LibError> {
         match DataWrapper::lookup_foreign(id) {
             Ok(foreign) => Ok(foreign),
             Err(_) => Ok(DataWrapper::Native(DataT { flag: false, val: id.len() as isize }))
         }
    }

    fn lookup_foreign(id: &str) -> Result<DataWrapper, LibError> {
        unsafe {
            let c_id = // null-terminated c-string
                CString::new(id)
                    .or(Err(LibError::BadObjId))?;

            let c_ptr = lookup_data(c_id.as_ptr());
            match c_ptr.is_null() {
                false => Ok(DataWrapper::Foreign(c_ptr)),
                true => Err(LibError::LookupFailed),
            }
        }
    }

    ///
    ///
    ///
    pub fn value(&self) -> Option<isize> {
        unsafe {
            match self {
                DataWrapper::Foreign(c_ptr) => Some((**c_ptr).val),
                DataWrapper::Native(data) => Some(data.val),
            }
        }
    }
}


///
///
///
impl Drop for DataWrapper {
    fn drop(&mut self) {
        match self {
            DataWrapper::Foreign(ptr) => {
                unsafe { free_data(*ptr) }
            },
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    ///
    ///
    #[test]
    fn foreign() {
        let obj: DataWrapper = DataWrapper::new("FOREIGN_A")
            .expect("data");

        assert_eq!(obj.value(), Some(1009));
    }

    ///
    ///
    ///
    #[test]
    fn native() {
        let obj: DataWrapper = DataWrapper::new("NATIVE")
            .expect("data");

        assert_eq!(obj.value(), Some(6));
    }
}
