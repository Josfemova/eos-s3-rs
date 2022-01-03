#[doc = "Register `APC_CTRL_1` reader"]
pub struct R(crate::R<APC_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APC_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APC_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APC_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APC_CTRL_1` writer"]
pub struct W(crate::W<APC_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APC_CTRL_1_SPEC>;
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
impl From<crate::W<APC_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APC_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tt` reader - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct TT_R(crate::FieldReader<u8, u8>);
impl TT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tt` writer - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct TT_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `vt` reader - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct VT_R(crate::FieldReader<u8, u8>);
impl VT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vt` writer - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
pub struct VT_W<'a> {
    w: &'a mut W,
}
impl<'a> VT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn tt(&self) -> TT_R {
        TT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn vt(&self) -> VT_R {
        VT_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn tt(&mut self) -> TT_W {
        TT_W { w: self }
    }
    #[doc = "Bits 3:7 - Please refer to the Technical Reference Manual for others (No SYNC Needed)"]
    #[inline(always)]
    pub fn vt(&mut self) -> VT_W {
        VT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APC control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apc_ctrl_1](index.html) module"]
pub struct APC_CTRL_1_SPEC;
impl crate::RegisterSpec for APC_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apc_ctrl_1::R](R) reader structure"]
impl crate::Readable for APC_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apc_ctrl_1::W](W) writer structure"]
impl crate::Writable for APC_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APC_CTRL_1 to value 0"]
impl crate::Resettable for APC_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
