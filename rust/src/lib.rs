use std::ffi::c_uint;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use wolfram_library_link::sys::{mint, MArgument, WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR};


pub fn custom_dot(left: &[isize], right: &[isize]) -> isize{
    left.iter().zip(right).map(|(&x, &y)| x * (y-1)).sum()
}


#[no_mangle]
pub unsafe extern "C" fn replace_part_nd(
    lib_data: WolframLibraryData,
    arg_count: mint,
    args: *mut MArgument,
    _res: MArgument,
) -> c_uint {

    if arg_count != 3 {
        return LIBRARY_FUNCTION_ERROR as c_uint;
    }

    let na_funs = *(*lib_data).numericarrayLibraryFunctions;
    let data = *(*args).numeric;
    let positions = *(*args.offset(1)).numeric;
    let values = *(*args.offset(2)).numeric;

    let get_numeric_type = na_funs.MNumericArray_getType.unwrap();
    let get_rank = na_funs.MNumericArray_getRank.unwrap();
    let get_pointer = na_funs.MNumericArray_getData.unwrap();
    let get_flatten_length = na_funs.MNumericArray_getFlattenedLength.unwrap();
    let get_dimension = na_funs.MNumericArray_getDimensions.unwrap();

    // if types do not match or position is not a 2d matrix or values is not a 1d list -> return error
    if (get_numeric_type(data) != get_numeric_type(values)) ||  (get_rank(positions) != 2) || (get_rank(values) != 1){
        return LIBRARY_FUNCTION_ERROR as c_uint;
    }


    let mut data_dimensions = from_raw_parts(get_dimension(data) as *const isize, get_rank(data) as usize).to_vec();

    // position must be 2d
    let rank = *get_dimension(positions).offset(1) as usize;

    // used in dot to just add the last element position instead of multiplying
    data_dimensions[rank-1] = 1;

    let position_data = from_raw_parts(get_pointer(positions) as *const isize, get_flatten_length(positions) as usize);
    let position_matrix = position_data.chunks_exact(rank);

    let values_array = from_raw_parts(get_pointer(values) as *const i64, get_flatten_length(values) as usize);
    let data_array = from_raw_parts_mut(get_pointer(data) as *mut i64,get_flatten_length(data) as usize);


    for (pos,&value) in position_matrix.zip(values_array) {
        data_array[custom_dot(&data_dimensions, pos) as usize] = value;
    }


    LIBRARY_NO_ERROR as c_uint
}




/*
// this function manipulate directly a nd list given the indices and values, indices should be 0 based index, mathematica stores data in a row major format
#[no_mangle]
pub unsafe extern "C" fn replace_part_direct(
    lib_data: WolframLibraryData,
    arg_count: mint,
    args: *mut MArgument,
    _res: MArgument,
) -> c_uint {


    if arg_count != 3 {
        return LIBRARY_FUNCTION_ERROR as c_uint;
    }

    let na_funs = *(*lib_data).numericarrayLibraryFunctions;
    let data = *(*args).numeric;
    let positions = *(*args.offset(1)).numeric;
    let values = *(*args.offset(2)).numeric;

    let get_numeric_type = na_funs.MNumericArray_getType.unwrap();
    let get_rank = na_funs.MNumericArray_getRank.unwrap();
    let get_pointer = na_funs.MNumericArray_getData.unwrap();
    let get_flatten_length = na_funs.MNumericArray_getFlattenedLength.unwrap();


    let data_pointer = get_pointer(data) as *mut i64;

    // if types do not match or values is not a 1d list -> return error
    if (get_numeric_type(data) != get_numeric_type(values)) || (get_rank(values) != 1){
        return LIBRARY_FUNCTION_ERROR as c_uint;
    }


    let values_array = from_raw_parts(get_pointer(values) as *const i64, get_flatten_length(values) as usize);
    let position_data = from_raw_parts(get_pointer(positions) as *const isize, get_flatten_length(positions) as usize);
    let data_array = from_raw_parts_mut(data_pointer,get_flatten_length(data) as usize);

    for (&position_index,&value) in  position_data.iter().zip(values_array) {

        data_array[position_index as usize] = value;

    }

    LIBRARY_NO_ERROR as c_uint
}*/