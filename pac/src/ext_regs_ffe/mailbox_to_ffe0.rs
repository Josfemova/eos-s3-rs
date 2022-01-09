#[doc = "Register `MAILBOX_TO_FFE0` reader"]
pub struct R(crate::R<MAILBOX_TO_FFE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAILBOX_TO_FFE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAILBOX_TO_FFE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAILBOX_TO_FFE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAILBOX_TO_FFE0` writer"]
pub struct W(crate::W<MAILBOX_TO_FFE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAILBOX_TO_FFE0_SPEC>;
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
impl From<crate::W<MAILBOX_TO_FFE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAILBOX_TO_FFE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAILBOX_TO_FFE0` reader - Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
pub struct MAILBOX_TO_FFE0_R(crate::FieldReader<u32, u32>);
impl MAILBOX_TO_FFE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MAILBOX_TO_FFE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAILBOX_TO_FFE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAILBOX_TO_FFE0` writer - Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
pub struct MAILBOX_TO_FFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_TO_FFE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
    #[inline(always)]
    pub fn mailbox_to_ffe0(&self) -> MAILBOX_TO_FFE0_R {
        MAILBOX_TO_FFE0_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
    #[inline(always)]
    pub fn mailbox_to_ffe0(&mut self) -> MAILBOX_TO_FFE0_W {
        MAILBOX_TO_FFE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mailbox_to_ffe0](index.html) module"]
pub struct MAILBOX_TO_FFE0_SPEC;
impl crate::RegisterSpec for MAILBOX_TO_FFE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mailbox_to_ffe0::R](R) reader structure"]
impl crate::Readable for MAILBOX_TO_FFE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mailbox_to_ffe0::W](W) writer structure"]
impl crate::Writable for MAILBOX_TO_FFE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAILBOX_TO_FFE0 to value 0"]
impl crate::Resettable for MAILBOX_TO_FFE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
