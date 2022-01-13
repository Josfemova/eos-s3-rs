#[doc = "Register `LPSD_CONFIG` reader"]
pub struct R(crate::R<LPSD_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSD_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSD_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSD_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSD_CONFIG` writer"]
pub struct W(crate::W<LPSD_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSD_CONFIG_SPEC>;
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
impl From<crate::W<LPSD_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSD_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPSD_THD` reader - LPSD threshold parameter"]
pub struct LPSD_THD_R(crate::FieldReader<u16, u16>);
impl LPSD_THD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LPSD_THD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_THD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_THD` writer - LPSD threshold parameter"]
pub struct LPSD_THD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_THD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `LPSD_RATIO_STOP` reader - LPSD threshold parameter"]
pub struct LPSD_RATIO_STOP_R(crate::FieldReader<u8, u8>);
impl LPSD_RATIO_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPSD_RATIO_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_RATIO_STOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_RATIO_STOP` writer - LPSD threshold parameter"]
pub struct LPSD_RATIO_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_RATIO_STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LPSD_RATIO_RUN` reader - LPSD run parameter"]
pub struct LPSD_RATIO_RUN_R(crate::FieldReader<u8, u8>);
impl LPSD_RATIO_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPSD_RATIO_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_RATIO_RUN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_RATIO_RUN` writer - LPSD run parameter"]
pub struct LPSD_RATIO_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_RATIO_RUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - LPSD threshold parameter"]
    #[inline(always)]
    pub fn lpsd_thd(&self) -> LPSD_THD_R {
        LPSD_THD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - LPSD threshold parameter"]
    #[inline(always)]
    pub fn lpsd_ratio_stop(&self) -> LPSD_RATIO_STOP_R {
        LPSD_RATIO_STOP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - LPSD run parameter"]
    #[inline(always)]
    pub fn lpsd_ratio_run(&self) -> LPSD_RATIO_RUN_R {
        LPSD_RATIO_RUN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPSD threshold parameter"]
    #[inline(always)]
    pub fn lpsd_thd(&mut self) -> LPSD_THD_W {
        LPSD_THD_W { w: self }
    }
    #[doc = "Bits 16:23 - LPSD threshold parameter"]
    #[inline(always)]
    pub fn lpsd_ratio_stop(&mut self) -> LPSD_RATIO_STOP_W {
        LPSD_RATIO_STOP_W { w: self }
    }
    #[doc = "Bits 24:31 - LPSD run parameter"]
    #[inline(always)]
    pub fn lpsd_ratio_run(&mut self) -> LPSD_RATIO_RUN_W {
        LPSD_RATIO_RUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPSD config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsd_config](index.html) module"]
pub struct LPSD_CONFIG_SPEC;
impl crate::RegisterSpec for LPSD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsd_config::R](R) reader structure"]
impl crate::Readable for LPSD_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsd_config::W](W) writer structure"]
impl crate::Writable for LPSD_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPSD_CONFIG to value 0x4d58_04b0"]
impl crate::Resettable for LPSD_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4d58_04b0
    }
}
