#[doc = "Register `APC_CTRL_2` reader"]
pub struct R(crate::R<APC_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APC_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APC_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APC_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APC_CTRL_2` writer"]
pub struct W(crate::W<APC_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APC_CTRL_2_SPEC>;
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
impl From<crate::W<APC_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APC_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `it` reader - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct IT_R(crate::FieldReader<u8, u8>);
impl IT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `it` writer - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct IT_W<'a> {
    w: &'a mut W,
}
impl<'a> IT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `test` reader - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct TEST_R(crate::FieldReader<u8, u8>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test` writer - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn it(&self) -> IT_R {
        IT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:6 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn it(&mut self) -> IT_W {
        IT_W { w: self }
    }
    #[doc = "Bits 3:6 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APC control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apc_ctrl_2](index.html) module"]
pub struct APC_CTRL_2_SPEC;
impl crate::RegisterSpec for APC_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apc_ctrl_2::R](R) reader structure"]
impl crate::Readable for APC_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apc_ctrl_2::W](W) writer structure"]
impl crate::Writable for APC_CTRL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APC_CTRL_2 to value 0"]
impl crate::Resettable for APC_CTRL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
