#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN_FFE0_ONCE` writer - When a '1' is written to this location, causes the FFE to execute one complete run of its algorithm; reads as 0"]
pub struct RUN_FFE0_ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_FFE0_ONCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RUN_FFE1` writer - When a '1' is written to this location, causes the FFE1 to start; reads as 0"]
pub struct RUN_FFE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_FFE1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RUN_SM0_ONCE` writer - When a '1' is written to this location, causes the SM0 to run once; reads as 0"]
pub struct RUN_SM0_ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_SM0_ONCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RUN_SM1_ONCE` writer - When a '1' is written to this location, causes the SM1 to run once; reads as 0"]
pub struct RUN_SM1_ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_SM1_ONCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - When a '1' is written to this location, causes the FFE to execute one complete run of its algorithm; reads as 0"]
    #[inline(always)]
    pub fn run_ffe0_once(&mut self) -> RUN_FFE0_ONCE_W {
        RUN_FFE0_ONCE_W { w: self }
    }
    #[doc = "Bit 1 - When a '1' is written to this location, causes the FFE1 to start; reads as 0"]
    #[inline(always)]
    pub fn run_ffe1(&mut self) -> RUN_FFE1_W {
        RUN_FFE1_W { w: self }
    }
    #[doc = "Bit 2 - When a '1' is written to this location, causes the SM0 to run once; reads as 0"]
    #[inline(always)]
    pub fn run_sm0_once(&mut self) -> RUN_SM0_ONCE_W {
        RUN_SM0_ONCE_W { w: self }
    }
    #[doc = "Bit 3 - When a '1' is written to this location, causes the SM1 to run once; reads as 0"]
    #[inline(always)]
    pub fn run_sm1_once(&mut self) -> RUN_SM1_ONCE_W {
        RUN_SM1_ONCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Commands for the Flexible Fusion Engine\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
