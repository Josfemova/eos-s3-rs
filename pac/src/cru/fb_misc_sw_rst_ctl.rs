#[doc = "Register `FB_MISC_SW_RST_CTL` reader"]
pub struct R(crate::R<FB_MISC_SW_RST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_MISC_SW_RST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_MISC_SW_RST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_MISC_SW_RST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_MISC_SW_RST_CTL` writer"]
pub struct W(crate::W<FB_MISC_SW_RST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_MISC_SW_RST_CTL_SPEC>;
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
impl From<crate::W<FB_MISC_SW_RST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_MISC_SW_RST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBWB_SW_RESET_A {
    #[doc = "1: Enable the software reset. FW need to disable it manually"]
    ENABLE = 1,
}
impl From<AHBWB_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: AHBWB_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBWB_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R40"]
pub struct AHBWB_SW_RESET_R(crate::FieldReader<bool, AHBWB_SW_RESET_A>);
impl AHBWB_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBWB_SW_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHBWB_SW_RESET_A> {
        match self.bits {
            true => Some(AHBWB_SW_RESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AHBWB_SW_RESET_A::ENABLE
    }
}
impl core::ops::Deref for AHBWB_SW_RESET_R {
    type Target = crate::FieldReader<bool, AHBWB_SW_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBWB_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R40"]
pub struct AHBWB_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBWB_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBWB_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AHBWB_SW_RESET_A::ENABLE)
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
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R41"]
pub type PFAFIFO1_SW_RESET_A = AHBWB_SW_RESET_A;
#[doc = "Field `PFAFIFO1_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R41"]
pub type PFAFIFO1_SW_RESET_R = AHBWB_SW_RESET_R;
#[doc = "Field `PFAFIFO1_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R41"]
pub struct PFAFIFO1_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PFAFIFO1_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFAFIFO1_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PFAFIFO1_SW_RESET_A::ENABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R40"]
    #[inline(always)]
    pub fn ahbwb_sw_reset(&self) -> AHBWB_SW_RESET_R {
        AHBWB_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R41"]
    #[inline(always)]
    pub fn pfafifo1_sw_reset(&self) -> PFAFIFO1_SW_RESET_R {
        PFAFIFO1_SW_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R40"]
    #[inline(always)]
    pub fn ahbwb_sw_reset(&mut self) -> AHBWB_SW_RESET_W {
        AHBWB_SW_RESET_W { w: self }
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually. ==> For R41"]
    #[inline(always)]
    pub fn pfafifo1_sw_reset(&mut self) -> PFAFIFO1_SW_RESET_W {
        PFAFIFO1_SW_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not specified. FAFIFO1, AHBWB Software Reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_misc_sw_rst_ctl](index.html) module"]
pub struct FB_MISC_SW_RST_CTL_SPEC;
impl crate::RegisterSpec for FB_MISC_SW_RST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_misc_sw_rst_ctl::R](R) reader structure"]
impl crate::Readable for FB_MISC_SW_RST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_misc_sw_rst_ctl::W](W) writer structure"]
impl crate::Writable for FB_MISC_SW_RST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_MISC_SW_RST_CTL to value 0"]
impl crate::Resettable for FB_MISC_SW_RST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
