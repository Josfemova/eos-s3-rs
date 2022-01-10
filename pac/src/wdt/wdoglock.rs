#[doc = "Register `WDOGLOCK` reader"]
pub struct R(crate::R<WDOGLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGLOCK` writer"]
pub struct W(crate::W<WDOGLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGLOCK_SPEC>;
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
impl From<crate::W<WDOGLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WDOGLOCK_A {
    #[doc = "0: Write access is enabled, not locked"]
    LOCKED = 0,
    #[doc = "1: Write access is disabled, locked"]
    NOT_LOCKED = 1,
}
impl From<WDOGLOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: WDOGLOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDOGLOCK` reader - The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
pub struct WDOGLOCK_R(crate::FieldReader<u32, WDOGLOCK_A>);
impl WDOGLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WDOGLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOGLOCK_A> {
        match self.bits {
            0 => Some(WDOGLOCK_A::LOCKED),
            1 => Some(WDOGLOCK_A::NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == WDOGLOCK_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        **self == WDOGLOCK_A::NOT_LOCKED
    }
}
impl core::ops::Deref for WDOGLOCK_R {
    type Target = crate::FieldReader<u32, WDOGLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGLOCK` writer - The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
pub struct WDOGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOGLOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write access is enabled, not locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WDOGLOCK_A::LOCKED)
    }
    #[doc = "Write access is disabled, locked"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(WDOGLOCK_A::NOT_LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
    #[inline(always)]
    pub fn wdoglock(&self) -> WDOGLOCK_R {
        WDOGLOCK_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
    #[inline(always)]
    pub fn wdoglock(&mut self) -> WDOGLOCK_W {
        WDOGLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdoglock](index.html) module"]
pub struct WDOGLOCK_SPEC;
impl crate::RegisterSpec for WDOGLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdoglock::R](R) reader structure"]
impl crate::Readable for WDOGLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdoglock::W](W) writer structure"]
impl crate::Writable for WDOGLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0"]
impl crate::Resettable for WDOGLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
