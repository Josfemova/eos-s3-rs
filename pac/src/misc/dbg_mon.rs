#[doc = "Register `DBG_MON` reader"]
pub struct R(crate::R<DBG_MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEBUG_MON` reader - Reads the debug status/information"]
pub struct DEBUG_MON_R(crate::FieldReader<u8, u8>);
impl DEBUG_MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_MON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Reads the debug status/information"]
    #[inline(always)]
    pub fn debug_mon(&self) -> DEBUG_MON_R {
        DEBUG_MON_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Debug Monitor information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_mon](index.html) module"]
pub struct DBG_MON_SPEC;
impl crate::RegisterSpec for DBG_MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_mon::R](R) reader structure"]
impl crate::Readable for DBG_MON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG_MON to value 0"]
impl crate::Resettable for DBG_MON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
