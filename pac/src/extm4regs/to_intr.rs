#[doc = "Register `TO_INTR` reader"]
pub struct R(crate::R<TO_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TO_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TO_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TO_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TO_INTR` writer"]
pub struct W(crate::W<TO_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TO_INTR_SPEC>;
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
impl From<crate::W<TO_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TO_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_INTR_TMR_MON` reader - "]
pub struct TO_INTR_TMR_MON_R(crate::FieldReader<bool, bool>);
impl TO_INTR_TMR_MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_INTR_TMR_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_INTR_TMR_MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_INTR_TMR_MON` writer - "]
pub struct TO_INTR_TMR_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_INTR_TMR_MON_W<'a> {
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
#[doc = "Field `TO_INTR_UART_MON` reader - "]
pub struct TO_INTR_UART_MON_R(crate::FieldReader<bool, bool>);
impl TO_INTR_UART_MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_INTR_UART_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_INTR_UART_MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_INTR_UART_MON` writer - "]
pub struct TO_INTR_UART_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_INTR_UART_MON_W<'a> {
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
#[doc = "Field `TO_INTR_AON` reader - "]
pub struct TO_INTR_AON_R(crate::FieldReader<bool, bool>);
impl TO_INTR_AON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_INTR_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_INTR_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_INTR_AON` writer - "]
pub struct TO_INTR_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_INTR_AON_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn to_intr_tmr_mon(&self) -> TO_INTR_TMR_MON_R {
        TO_INTR_TMR_MON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn to_intr_uart_mon(&self) -> TO_INTR_UART_MON_R {
        TO_INTR_UART_MON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn to_intr_aon(&self) -> TO_INTR_AON_R {
        TO_INTR_AON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn to_intr_tmr_mon(&mut self) -> TO_INTR_TMR_MON_W {
        TO_INTR_TMR_MON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn to_intr_uart_mon(&mut self) -> TO_INTR_UART_MON_W {
        TO_INTR_UART_MON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn to_intr_aon(&mut self) -> TO_INTR_AON_W {
        TO_INTR_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description provided\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to_intr](index.html) module"]
pub struct TO_INTR_SPEC;
impl crate::RegisterSpec for TO_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [to_intr::R](R) reader structure"]
impl crate::Readable for TO_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [to_intr::W](W) writer structure"]
impl crate::Writable for TO_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TO_INTR to value 0"]
impl crate::Resettable for TO_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
