#[doc = "Register `DST_DATA_END_PTR_CH12` reader"]
pub struct R(crate::R<DST_DATA_END_PTR_CH12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_DATA_END_PTR_CH12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_DATA_END_PTR_CH12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_DATA_END_PTR_CH12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST_DATA_END_PTR_CH12` writer"]
pub struct W(crate::W<DST_DATA_END_PTR_CH12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_DATA_END_PTR_CH12_SPEC>;
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
impl From<crate::W<DST_DATA_END_PTR_CH12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_DATA_END_PTR_CH12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST_DATA_END_PTR_CH12` reader - Primary pointer to the end address of the destination data of channel 12"]
pub struct DST_DATA_END_PTR_CH12_R(crate::FieldReader<u32, u32>);
impl DST_DATA_END_PTR_CH12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DST_DATA_END_PTR_CH12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_DATA_END_PTR_CH12_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_DATA_END_PTR_CH12` writer - Primary pointer to the end address of the destination data of channel 12"]
pub struct DST_DATA_END_PTR_CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_DATA_END_PTR_CH12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Primary pointer to the end address of the destination data of channel 12"]
    #[inline(always)]
    pub fn dst_data_end_ptr_ch12(&self) -> DST_DATA_END_PTR_CH12_R {
        DST_DATA_END_PTR_CH12_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Primary pointer to the end address of the destination data of channel 12"]
    #[inline(always)]
    pub fn dst_data_end_ptr_ch12(&mut self) -> DST_DATA_END_PTR_CH12_W {
        DST_DATA_END_PTR_CH12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Primary pointer to the end address of the destination data of channel 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_data_end_ptr_ch12](index.html) module"]
pub struct DST_DATA_END_PTR_CH12_SPEC;
impl crate::RegisterSpec for DST_DATA_END_PTR_CH12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_data_end_ptr_ch12::R](R) reader structure"]
impl crate::Readable for DST_DATA_END_PTR_CH12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst_data_end_ptr_ch12::W](W) writer structure"]
impl crate::Writable for DST_DATA_END_PTR_CH12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DST_DATA_END_PTR_CH12 to value 0"]
impl crate::Resettable for DST_DATA_END_PTR_CH12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
