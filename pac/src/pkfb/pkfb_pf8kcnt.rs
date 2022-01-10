#[doc = "Register `PKFB_PF8KCNT` reader"]
pub struct R(crate::R<PKFB_PF8KCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF8KCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF8KCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF8KCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pf8k_pop_cnt` reader - FIFO 8k Pop Count (x16 count) Number of available entries for pop"]
pub struct PF8K_POP_CNT_R(crate::FieldReader<u16, u16>);
impl PF8K_POP_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PF8K_POP_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_POP_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_empty` reader - FIFO 8k Empty"]
pub struct PF8K_EMPTY_R(crate::FieldReader<bool, bool>);
impl PF8K_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_push_cnt` reader - FIFO 8k Push Count (x32 count) Number of available entries for push"]
pub struct PF8K_PUSH_CNT_R(crate::FieldReader<u16, u16>);
impl PF8K_PUSH_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PF8K_PUSH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_PUSH_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_full` reader - FIFO 8k Full"]
pub struct PF8K_FULL_R(crate::FieldReader<bool, bool>);
impl PF8K_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - FIFO 8k Pop Count (x16 count) Number of available entries for pop"]
    #[inline(always)]
    pub fn pf8k_pop_cnt(&self) -> PF8K_POP_CNT_R {
        PF8K_POP_CNT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - FIFO 8k Empty"]
    #[inline(always)]
    pub fn pf8k_empty(&self) -> PF8K_EMPTY_R {
        PF8K_EMPTY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:28 - FIFO 8k Push Count (x32 count) Number of available entries for push"]
    #[inline(always)]
    pub fn pf8k_push_cnt(&self) -> PF8K_PUSH_CNT_R {
        PF8K_PUSH_CNT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - FIFO 8k Full"]
    #[inline(always)]
    pub fn pf8k_full(&self) -> PF8K_FULL_R {
        PF8K_FULL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "FIFO 8k Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf8kcnt](index.html) module"]
pub struct PKFB_PF8KCNT_SPEC;
impl crate::RegisterSpec for PKFB_PF8KCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf8kcnt::R](R) reader structure"]
impl crate::Readable for PKFB_PF8KCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKFB_PF8KCNT to value 0x0100_8000"]
impl crate::Resettable for PKFB_PF8KCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_8000
    }
}
