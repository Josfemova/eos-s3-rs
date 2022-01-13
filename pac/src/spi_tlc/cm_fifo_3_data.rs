#[doc = "Register `CM_FIFO_3_DATA` reader"]
pub struct R(crate::R<CM_FIFO_3_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM_FIFO_3_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM_FIFO_3_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM_FIFO_3_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - PF FIFO Read Port"]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PF FIFO Read Port"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits as u8)
    }
}
#[doc = "PF Bank FIFO 3 Read Port\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm_fifo_3_data](index.html) module"]
pub struct CM_FIFO_3_DATA_SPEC;
impl crate::RegisterSpec for CM_FIFO_3_DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cm_fifo_3_data::R](R) reader structure"]
impl crate::Readable for CM_FIFO_3_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM_FIFO_3_DATA to value 0"]
impl crate::Resettable for CM_FIFO_3_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
