#[doc = "Register `ALT_SRC_DATA_END_PTR_CH3` reader"]
pub struct R(crate::R<ALT_SRC_DATA_END_PTR_CH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALT_SRC_DATA_END_PTR_CH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALT_SRC_DATA_END_PTR_CH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALT_SRC_DATA_END_PTR_CH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALT_SRC_DATA_END_PTR_CH3` writer"]
pub struct W(crate::W<ALT_SRC_DATA_END_PTR_CH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALT_SRC_DATA_END_PTR_CH3_SPEC>;
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
impl From<crate::W<ALT_SRC_DATA_END_PTR_CH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALT_SRC_DATA_END_PTR_CH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALT_SRC_DATA_END_PTR_CH3` reader - Alternate pointer to the end address of the source data of channel 3"]
pub struct ALT_SRC_DATA_END_PTR_CH3_R(crate::FieldReader<u32, u32>);
impl ALT_SRC_DATA_END_PTR_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ALT_SRC_DATA_END_PTR_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALT_SRC_DATA_END_PTR_CH3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALT_SRC_DATA_END_PTR_CH3` writer - Alternate pointer to the end address of the source data of channel 3"]
pub struct ALT_SRC_DATA_END_PTR_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_SRC_DATA_END_PTR_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alternate pointer to the end address of the source data of channel 3"]
    #[inline(always)]
    pub fn alt_src_data_end_ptr_ch3(&self) -> ALT_SRC_DATA_END_PTR_CH3_R {
        ALT_SRC_DATA_END_PTR_CH3_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alternate pointer to the end address of the source data of channel 3"]
    #[inline(always)]
    pub fn alt_src_data_end_ptr_ch3(&mut self) -> ALT_SRC_DATA_END_PTR_CH3_W {
        ALT_SRC_DATA_END_PTR_CH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate pointer to the end address of the source data of channel 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_src_data_end_ptr_ch3](index.html) module"]
pub struct ALT_SRC_DATA_END_PTR_CH3_SPEC;
impl crate::RegisterSpec for ALT_SRC_DATA_END_PTR_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alt_src_data_end_ptr_ch3::R](R) reader structure"]
impl crate::Readable for ALT_SRC_DATA_END_PTR_CH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alt_src_data_end_ptr_ch3::W](W) writer structure"]
impl crate::Writable for ALT_SRC_DATA_END_PTR_CH3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALT_SRC_DATA_END_PTR_CH3 to value 0"]
impl crate::Resettable for ALT_SRC_DATA_END_PTR_CH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
