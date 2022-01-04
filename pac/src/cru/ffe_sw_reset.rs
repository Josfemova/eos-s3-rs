#[doc = "Register `FFE_SW_RESET` reader"]
pub struct R(crate::R<FFE_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_SW_RESET` writer"]
pub struct W(crate::W<FFE_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_SW_RESET_SPEC>;
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
impl From<crate::W<FFE_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE_0_X1_SW_RESET_A {
    #[doc = "1: Enable the software reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    ENABLE = 1,
}
impl From<FFE_0_X1_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: FFE_0_X1_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE_0_X1_SW_Reset` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
pub struct FFE_0_X1_SW_RESET_R(crate::FieldReader<bool, FFE_0_X1_SW_RESET_A>);
impl FFE_0_X1_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_0_X1_SW_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFE_0_X1_SW_RESET_A> {
        match self.bits {
            true => Some(FFE_0_X1_SW_RESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FFE_0_X1_SW_RESET_A::ENABLE
    }
}
impl core::ops::Deref for FFE_0_X1_SW_RESET_R {
    type Target = crate::FieldReader<bool, FFE_0_X1_SW_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_0_X1_SW_Reset` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
pub struct FFE_0_X1_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_0_X1_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_0_X1_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE_0_X1_SW_RESET_A::ENABLE)
    }
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
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE_0_X4_SW_RESET_A {
    #[doc = "1: Enable the software reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    ENABLE = 1,
}
impl From<FFE_0_X4_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: FFE_0_X4_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE_0_X4_SW_Reset` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
pub struct FFE_0_X4_SW_RESET_R(crate::FieldReader<bool, FFE_0_X4_SW_RESET_A>);
impl FFE_0_X4_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_0_X4_SW_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFE_0_X4_SW_RESET_A> {
        match self.bits {
            true => Some(FFE_0_X4_SW_RESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FFE_0_X4_SW_RESET_A::ENABLE
    }
}
impl core::ops::Deref for FFE_0_X4_SW_RESET_R {
    type Target = crate::FieldReader<bool, FFE_0_X4_SW_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_0_X4_SW_Reset` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
pub struct FFE_0_X4_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_0_X4_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_0_X4_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE_0_X4_SW_RESET_A::ENABLE)
    }
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
impl R {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn ffe_0_x1_sw_reset(&self) -> FFE_0_X1_SW_RESET_R {
        FFE_0_X1_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn ffe_0_x4_sw_reset(&self) -> FFE_0_X4_SW_RESET_R {
        FFE_0_X4_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn ffe_0_x1_sw_reset(&mut self) -> FFE_0_X1_SW_RESET_W {
        FFE_0_X1_SW_RESET_W { w: self }
    }
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. (R01_P3_FFE as well)"]
    #[inline(always)]
    pub fn ffe_0_x4_sw_reset(&mut self) -> FFE_0_X4_SW_RESET_W {
        FFE_0_X4_SW_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_sw_reset](index.html) module"]
pub struct FFE_SW_RESET_SPEC;
impl crate::RegisterSpec for FFE_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_sw_reset::R](R) reader structure"]
impl crate::Readable for FFE_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_sw_reset::W](W) writer structure"]
impl crate::Writable for FFE_SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_SW_RESET to value 0"]
impl crate::Resettable for FFE_SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
