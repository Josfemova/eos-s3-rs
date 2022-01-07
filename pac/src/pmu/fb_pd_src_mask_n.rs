#[doc = "Register `FB_PD_SRC_MASK_N` reader"]
pub struct R(crate::R<FB_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_PD_SRC_MASK_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_PD_SRC_MASK_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_PD_SRC_MASK_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_PD_SRC_MASK_N` writer"]
pub struct W(crate::W<FB_PD_SRC_MASK_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_PD_SRC_MASK_N_SPEC>;
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
impl From<crate::W<FB_PD_SRC_MASK_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_PD_SRC_MASK_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control the mask for the Interface Signal 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERFACE_SIGNAL_0_A {
    #[doc = "0: Mask the signal as a power-down event source"]
    MASK = 0,
    #[doc = "1: Unmask the signal as a power-down event source"]
    UNMASK = 1,
}
impl From<INTERFACE_SIGNAL_0_A> for bool {
    #[inline(always)]
    fn from(variant: INTERFACE_SIGNAL_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Interface_Signal_0` reader - Control the mask for the Interface Signal 0"]
pub struct INTERFACE_SIGNAL_0_R(crate::FieldReader<bool, INTERFACE_SIGNAL_0_A>);
impl INTERFACE_SIGNAL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERFACE_SIGNAL_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERFACE_SIGNAL_0_A {
        match self.bits {
            false => INTERFACE_SIGNAL_0_A::MASK,
            true => INTERFACE_SIGNAL_0_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == INTERFACE_SIGNAL_0_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == INTERFACE_SIGNAL_0_A::UNMASK
    }
}
impl core::ops::Deref for INTERFACE_SIGNAL_0_R {
    type Target = crate::FieldReader<bool, INTERFACE_SIGNAL_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Interface_Signal_0` writer - Control the mask for the Interface Signal 0"]
pub struct INTERFACE_SIGNAL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_SIGNAL_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_SIGNAL_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the signal as a power-down event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_0_A::MASK)
    }
    #[doc = "Unmask the signal as a power-down event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_0_A::UNMASK)
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
#[doc = "Control the mask for the Interface Signal 1"]
pub type INTERFACE_SIGNAL_1_A = INTERFACE_SIGNAL_0_A;
#[doc = "Field `Interface_Signal_1` reader - Control the mask for the Interface Signal 1"]
pub type INTERFACE_SIGNAL_1_R = INTERFACE_SIGNAL_0_R;
#[doc = "Field `Interface_Signal_1` writer - Control the mask for the Interface Signal 1"]
pub struct INTERFACE_SIGNAL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_SIGNAL_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_SIGNAL_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the signal as a power-down event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_1_A::MASK)
    }
    #[doc = "Unmask the signal as a power-down event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_1_A::UNMASK)
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Control the mask for the Interface Signal 2"]
pub type INTERFACE_SIGNAL_2_A = INTERFACE_SIGNAL_0_A;
#[doc = "Field `Interface_Signal_2` reader - Control the mask for the Interface Signal 2"]
pub type INTERFACE_SIGNAL_2_R = INTERFACE_SIGNAL_0_R;
#[doc = "Field `Interface_Signal_2` writer - Control the mask for the Interface Signal 2"]
pub struct INTERFACE_SIGNAL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_SIGNAL_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_SIGNAL_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the signal as a power-down event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_2_A::MASK)
    }
    #[doc = "Unmask the signal as a power-down event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_2_A::UNMASK)
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Control the mask for the Interface Signal 3"]
pub type INTERFACE_SIGNAL_3_A = INTERFACE_SIGNAL_0_A;
#[doc = "Field `Interface_Signal_3` reader - Control the mask for the Interface Signal 3"]
pub type INTERFACE_SIGNAL_3_R = INTERFACE_SIGNAL_0_R;
#[doc = "Field `Interface_Signal_3` writer - Control the mask for the Interface Signal 3"]
pub struct INTERFACE_SIGNAL_3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_SIGNAL_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_SIGNAL_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the signal as a power-down event source"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_3_A::MASK)
    }
    #[doc = "Unmask the signal as a power-down event source"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INTERFACE_SIGNAL_3_A::UNMASK)
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Control the mask for the Interface Signal 0"]
    #[inline(always)]
    pub fn interface_signal_0(&self) -> INTERFACE_SIGNAL_0_R {
        INTERFACE_SIGNAL_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control the mask for the Interface Signal 1"]
    #[inline(always)]
    pub fn interface_signal_1(&self) -> INTERFACE_SIGNAL_1_R {
        INTERFACE_SIGNAL_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control the mask for the Interface Signal 2"]
    #[inline(always)]
    pub fn interface_signal_2(&self) -> INTERFACE_SIGNAL_2_R {
        INTERFACE_SIGNAL_2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control the mask for the Interface Signal 3"]
    #[inline(always)]
    pub fn interface_signal_3(&self) -> INTERFACE_SIGNAL_3_R {
        INTERFACE_SIGNAL_3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Control the mask for the Interface Signal 0"]
    #[inline(always)]
    pub fn interface_signal_0(&mut self) -> INTERFACE_SIGNAL_0_W {
        INTERFACE_SIGNAL_0_W { w: self }
    }
    #[doc = "Bit 2 - Control the mask for the Interface Signal 1"]
    #[inline(always)]
    pub fn interface_signal_1(&mut self) -> INTERFACE_SIGNAL_1_W {
        INTERFACE_SIGNAL_1_W { w: self }
    }
    #[doc = "Bit 3 - Control the mask for the Interface Signal 2"]
    #[inline(always)]
    pub fn interface_signal_2(&mut self) -> INTERFACE_SIGNAL_2_W {
        INTERFACE_SIGNAL_2_W { w: self }
    }
    #[doc = "Bit 4 - Control the mask for the Interface Signal 3"]
    #[inline(always)]
    pub fn interface_signal_3(&mut self) -> INTERFACE_SIGNAL_3_W {
        INTERFACE_SIGNAL_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control masking of power down event signals for the FPGA Fabric power domain. The falling edge of any of the listed signals (non-mask) will put the FB into Power saving mode base on the Power Mode Cfg. Note: These signals used to indicate the BUSY status, so they must be level signals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_pd_src_mask_n](index.html) module"]
pub struct FB_PD_SRC_MASK_N_SPEC;
impl crate::RegisterSpec for FB_PD_SRC_MASK_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_pd_src_mask_n::R](R) reader structure"]
impl crate::Readable for FB_PD_SRC_MASK_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_pd_src_mask_n::W](W) writer structure"]
impl crate::Writable for FB_PD_SRC_MASK_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_PD_SRC_MASK_N to value 0"]
impl crate::Resettable for FB_PD_SRC_MASK_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
