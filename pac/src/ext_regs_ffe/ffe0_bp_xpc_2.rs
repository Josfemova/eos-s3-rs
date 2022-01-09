#[doc = "Register `FFE0_BP_XPC_2` reader"]
pub struct R(crate::R<FFE0_BP_XPC_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE0_BP_XPC_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE0_BP_XPC_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE0_BP_XPC_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE0_BP_XPC_2` writer"]
pub struct W(crate::W<FFE0_BP_XPC_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE0_BP_XPC_2_SPEC>;
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
impl From<crate::W<FFE0_BP_XPC_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE0_BP_XPC_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFE0_BP_XPC_2` reader - These registers hold the xPC (program counter) address 'break points'."]
pub struct FFE0_BP_XPC_2_R(crate::FieldReader<u16, u16>);
impl FFE0_BP_XPC_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FFE0_BP_XPC_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BP_XPC_2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BP_XPC_2` writer - These registers hold the xPC (program counter) address 'break points'."]
pub struct FFE0_BP_XPC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BP_XPC_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - These registers hold the xPC (program counter) address 'break points'."]
    #[inline(always)]
    pub fn ffe0_bp_xpc_2(&self) -> FFE0_BP_XPC_2_R {
        FFE0_BP_XPC_2_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - These registers hold the xPC (program counter) address 'break points'."]
    #[inline(always)]
    pub fn ffe0_bp_xpc_2(&mut self) -> FFE0_BP_XPC_2_W {
        FFE0_BP_XPC_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "These registers hold the xPC (program counter) address 'break points'.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe0_bp_xpc_2](index.html) module"]
pub struct FFE0_BP_XPC_2_SPEC;
impl crate::RegisterSpec for FFE0_BP_XPC_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe0_bp_xpc_2::R](R) reader structure"]
impl crate::Readable for FFE0_BP_XPC_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe0_bp_xpc_2::W](W) writer structure"]
impl crate::Writable for FFE0_BP_XPC_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE0_BP_XPC_2 to value 0"]
impl crate::Resettable for FFE0_BP_XPC_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
