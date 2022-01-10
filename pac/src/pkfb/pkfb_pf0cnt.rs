#[doc = "Register `PKFB_PF0CNT` reader"]
pub struct R(crate::R<PKFB_PF0CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF0CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF0CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF0CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pf0_pop_cnt` reader - FIFO 0 Pop Count (x32 count) Number of available entries for pop"]
pub struct PF0_POP_CNT_R(crate::FieldReader<u16, u16>);
impl PF0_POP_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PF0_POP_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_empty` reader - FIFO 0 Empty"]
pub struct PF0_EMPTY_R(crate::FieldReader<bool, bool>);
impl PF0_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_cnt` reader - FIFO 0 Push Count (x32 count) Number of available entries for push"]
pub struct PF0_PUSH_CNT_R(crate::FieldReader<u16, u16>);
impl PF0_PUSH_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PF0_PUSH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_PUSH_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_full` reader - FIFO 0 Full"]
pub struct PF0_FULL_R(crate::FieldReader<bool, bool>);
impl PF0_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - FIFO 0 Pop Count (x32 count) Number of available entries for pop"]
    #[inline(always)]
    pub fn pf0_pop_cnt(&self) -> PF0_POP_CNT_R {
        PF0_POP_CNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - FIFO 0 Empty"]
    #[inline(always)]
    pub fn pf0_empty(&self) -> PF0_EMPTY_R {
        PF0_EMPTY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - FIFO 0 Push Count (x32 count) Number of available entries for push"]
    #[inline(always)]
    pub fn pf0_push_cnt(&self) -> PF0_PUSH_CNT_R {
        PF0_PUSH_CNT_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - FIFO 0 Full"]
    #[inline(always)]
    pub fn pf0_full(&self) -> PF0_FULL_R {
        PF0_FULL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "FIFO 0 Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf0cnt](index.html) module"]
pub struct PKFB_PF0CNT_SPEC;
impl crate::RegisterSpec for PKFB_PF0CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf0cnt::R](R) reader structure"]
impl crate::Readable for PKFB_PF0CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKFB_PF0CNT to value 0x0100_8000"]
impl crate::Resettable for PKFB_PF0CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_8000
    }
}
