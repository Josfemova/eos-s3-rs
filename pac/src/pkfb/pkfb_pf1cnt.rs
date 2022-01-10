#[doc = "Register `PKFB_PF1CNT` reader"]
pub struct R(crate::R<PKFB_PF1CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF1CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF1CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF1CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pf1_pop_cnt` reader - FIFO 1 Pop Count (x32 count) Number of available entries for pop"]
pub struct PF1_POP_CNT_R(crate::FieldReader<u8, u8>);
impl PF1_POP_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF1_POP_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_POP_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_empty` reader - FIFO 1 Empty"]
pub struct PF1_EMPTY_R(crate::FieldReader<bool, bool>);
impl PF1_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_push_cnt` reader - FIFO 1 Push Count (x32 count) Number of available entries for push"]
pub struct PF1_PUSH_CNT_R(crate::FieldReader<u8, u8>);
impl PF1_PUSH_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF1_PUSH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_PUSH_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_full` reader - FIFO 1 Full"]
pub struct PF1_FULL_R(crate::FieldReader<bool, bool>);
impl PF1_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO 1 Pop Count (x32 count) Number of available entries for pop"]
    #[inline(always)]
    pub fn pf1_pop_cnt(&self) -> PF1_POP_CNT_R {
        PF1_POP_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - FIFO 1 Empty"]
    #[inline(always)]
    pub fn pf1_empty(&self) -> PF1_EMPTY_R {
        PF1_EMPTY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - FIFO 1 Push Count (x32 count) Number of available entries for push"]
    #[inline(always)]
    pub fn pf1_push_cnt(&self) -> PF1_PUSH_CNT_R {
        PF1_PUSH_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Full"]
    #[inline(always)]
    pub fn pf1_full(&self) -> PF1_FULL_R {
        PF1_FULL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "FIFO 1 Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf1cnt](index.html) module"]
pub struct PKFB_PF1CNT_SPEC;
impl crate::RegisterSpec for PKFB_PF1CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf1cnt::R](R) reader structure"]
impl crate::Readable for PKFB_PF1CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKFB_PF1CNT to value 0x0080_8000"]
impl crate::Resettable for PKFB_PF1CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_8000
    }
}
