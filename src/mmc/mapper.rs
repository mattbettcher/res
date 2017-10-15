// simple trait for mapper operations
pub trait Mapper {
    fn read(&mut self, a: usize) -> u8;
    fn write(&mut self, a: usize, v: u8); 

    // todo - other mapper interface stuff?
    //          interupts?
    //          counters?
    //          mirroring setup?
    //          what else needs to be here to support every mapper?
}