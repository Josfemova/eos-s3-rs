#[doc = "Register `SRC_DATA_END_PTR_CH10` reader"]
pub struct R(crate::R<SRC_DATA_END_PTR_CH10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_DATA_END_PTR_CH10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_DATA_END_PTR_CH10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_DATA_END_PTR_CH10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_DATA_END_PTR_CH10` writer"]
pub struct W(crate::W<SRC_DATA_END_PTR_CH10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_DATA_END_PTR_CH10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SRC_DATA_END_PTR_CH10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_DATA_END_PTR_CH10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_DATA_END_PTR_CH10` reader - Primary pointer to the end address of the source data of channel 10"]
pub struct SRC_DATA_END_PTR_CH10_R(crate::FieldReader<u32, u32>);
impl SRC_DATA_END_PTR_CH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRC_DATA_END_PTR_CH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_DATA_END_PTR_CH10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_DATA_END_PTR_CH10` writer - Primary pointer to the end address of the source data of channel 10"]
pub struct SRC_DATA_END_PTR_CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_DATA_END_PTR_CH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Primary pointer to the end address of the source data of channel 10"]
    #[inline(always)]
    pub fn src_data_end_ptr_ch10(&self) -> SRC_DATA_END_PTR_CH10_R {
        SRC_DATA_END_PTR_CH10_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Primary pointer to the end address of the source data of channel 10"]
    #[inline(always)]
    pub fn src_data_end_ptr_ch10(&mut self) -> SRC_DATA_END_PTR_CH10_W {
        SRC_DATA_END_PTR_CH10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Primary pointer to the end address of the source data of channel 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_data_end_ptr_ch10](index.html) module"]
pub struct SRC_DATA_END_PTR_CH10_SPEC;
impl crate::RegisterSpec for SRC_DATA_END_PTR_CH10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_data_end_ptr_ch10::R](R) reader structure"]
impl crate::Readable for SRC_DATA_END_PTR_CH10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_data_end_ptr_ch10::W](W) writer structure"]
impl crate::Writable for SRC_DATA_END_PTR_CH10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC_DATA_END_PTR_CH10 to value 0"]
impl crate::Resettable for SRC_DATA_END_PTR_CH10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
