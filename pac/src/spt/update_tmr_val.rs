#[doc = "Register `UPDATE_TMR_VAL` reader"]
pub struct R(crate::R<UPDATE_TMR_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_TMR_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_TMR_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_TMR_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE_TMR_VAL` writer"]
pub struct W(crate::W<UPDATE_TMR_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_TMR_VAL_SPEC>;
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
impl From<crate::W<UPDATE_TMR_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_TMR_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE_TIMER_VALUE` reader - Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
pub struct UPDATE_TIMER_VALUE_R(crate::FieldReader<u32, u32>);
impl UPDATE_TIMER_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UPDATE_TIMER_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDATE_TIMER_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE_TIMER_VALUE` writer - Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
pub struct UPDATE_TIMER_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_TIMER_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
    #[inline(always)]
    pub fn update_timer_value(&self) -> UPDATE_TIMER_VALUE_R {
        UPDATE_TIMER_VALUE_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue"]
    #[inline(always)]
    pub fn update_timer_value(&mut self) -> UPDATE_TIMER_VALUE_W {
        UPDATE_TIMER_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Update the 30-Bit Timer once write. Note: Please programmed SPT_EN (0x000, bit 0) to 0 before write this register to avoid any potential issue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_tmr_val](index.html) module"]
pub struct UPDATE_TMR_VAL_SPEC;
impl crate::RegisterSpec for UPDATE_TMR_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update_tmr_val::R](R) reader structure"]
impl crate::Readable for UPDATE_TMR_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update_tmr_val::W](W) writer structure"]
impl crate::Writable for UPDATE_TMR_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPDATE_TMR_VAL to value 0"]
impl crate::Resettable for UPDATE_TMR_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
