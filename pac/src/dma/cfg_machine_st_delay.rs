#[doc = "Register `CFG_MACHINE_ST_DELAY` reader"]
pub struct R(crate::R<CFG_MACHINE_ST_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_MACHINE_ST_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_MACHINE_ST_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_MACHINE_ST_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_MACHINE_ST_DELAY` writer"]
pub struct W(crate::W<CFG_MACHINE_ST_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_MACHINE_ST_DELAY_SPEC>;
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
impl From<crate::W<CFG_MACHINE_ST_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_MACHINE_ST_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `delay_reg` reader - Delay value used in the config SM state machine. This is clocked at the APB pclk frequency. Default to decimal 500."]
pub struct DELAY_REG_R(crate::FieldReader<u16, u16>);
impl DELAY_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DELAY_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_REG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `delay_reg` writer - Delay value used in the config SM state machine. This is clocked at the APB pclk frequency. Default to decimal 500."]
pub struct DELAY_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Delay value used in the config SM state machine. This is clocked at the APB pclk frequency. Default to decimal 500."]
    #[inline(always)]
    pub fn delay_reg(&self) -> DELAY_REG_R {
        DELAY_REG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay value used in the config SM state machine. This is clocked at the APB pclk frequency. Default to decimal 500."]
    #[inline(always)]
    pub fn delay_reg(&mut self) -> DELAY_REG_W {
        DELAY_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the delay value used in the config state machine. It is used for both deep sleep wakeup delay and between retries.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_machine_st_delay](index.html) module"]
pub struct CFG_MACHINE_ST_DELAY_SPEC;
impl crate::RegisterSpec for CFG_MACHINE_ST_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_machine_st_delay::R](R) reader structure"]
impl crate::Readable for CFG_MACHINE_ST_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_machine_st_delay::W](W) writer structure"]
impl crate::Writable for CFG_MACHINE_ST_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_MACHINE_ST_DELAY to value 0x01f4"]
impl crate::Resettable for CFG_MACHINE_ST_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01f4
    }
}
