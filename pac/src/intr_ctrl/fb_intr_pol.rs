#[doc = "Register `FB_INTR_POL` reader"]
pub struct R(crate::R<FB_INTR_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_INTR_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_INTR_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_INTR_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_INTR_POL` writer"]
pub struct W(crate::W<FB_INTR_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_INTR_POL_SPEC>;
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
impl From<crate::W<FB_INTR_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_INTR_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FB_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB_0_INTR_POL_A {
    #[doc = "0: Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    LOW_FALL = 0,
    #[doc = "1: Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    HIGH_RISE = 1,
}
impl From<FB_0_INTR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: FB_0_INTR_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FB_0_INTR_POL` reader - FB_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct FB_0_INTR_POL_R(crate::FieldReader<bool, FB_0_INTR_POL_A>);
impl FB_0_INTR_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_0_INTR_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_0_INTR_POL_A {
        match self.bits {
            false => FB_0_INTR_POL_A::LOW_FALL,
            true => FB_0_INTR_POL_A::HIGH_RISE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_FALL`"]
    #[inline(always)]
    pub fn is_low_fall(&self) -> bool {
        **self == FB_0_INTR_POL_A::LOW_FALL
    }
    #[doc = "Checks if the value of the field is `HIGH_RISE`"]
    #[inline(always)]
    pub fn is_high_rise(&self) -> bool {
        **self == FB_0_INTR_POL_A::HIGH_RISE
    }
}
impl core::ops::Deref for FB_0_INTR_POL_R {
    type Target = crate::FieldReader<bool, FB_0_INTR_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_0_INTR_POL` writer - FB_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct FB_0_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_0_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_0_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(FB_0_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(FB_0_INTR_POL_A::HIGH_RISE)
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
#[doc = "FB_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_1_INTR_POL_A = FB_0_INTR_POL_A;
#[doc = "Field `FB_1_INTR_POL` reader - FB_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_1_INTR_POL_R = FB_0_INTR_POL_R;
#[doc = "Field `FB_1_INTR_POL` writer - FB_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct FB_1_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_1_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_1_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(FB_1_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(FB_1_INTR_POL_A::HIGH_RISE)
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
#[doc = "FB_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_2_INTR_POL_A = FB_0_INTR_POL_A;
#[doc = "Field `FB_2_INTR_POL` reader - FB_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_2_INTR_POL_R = FB_0_INTR_POL_R;
#[doc = "Field `FB_2_INTR_POL` writer - FB_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct FB_2_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_2_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_2_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(FB_2_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(FB_2_INTR_POL_A::HIGH_RISE)
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
#[doc = "FB_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_3_INTR_POL_A = FB_0_INTR_POL_A;
#[doc = "Field `FB_3_INTR_POL` reader - FB_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub type FB_3_INTR_POL_R = FB_0_INTR_POL_R;
#[doc = "Field `FB_3_INTR_POL` writer - FB_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
pub struct FB_3_INTR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_3_INTR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_3_INTR_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading low value, if it is an edge trigger, this value indicates that the interrupt polarity is falling edge"]
    #[inline(always)]
    pub fn low_fall(self) -> &'a mut W {
        self.variant(FB_3_INTR_POL_A::LOW_FALL)
    }
    #[doc = "Depending on the type, if the interrupt is triggered by level, this indicates the interrupt is triggered by a reading high value, if it is an edge trigger, this value indicates that the interrupt polarity is rising edge"]
    #[inline(always)]
    pub fn high_rise(self) -> &'a mut W {
        self.variant(FB_3_INTR_POL_A::HIGH_RISE)
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
impl R {
    #[doc = "Bit 0 - FB_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_0_intr_pol(&self) -> FB_0_INTR_POL_R {
        FB_0_INTR_POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FB_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_1_intr_pol(&self) -> FB_1_INTR_POL_R {
        FB_1_INTR_POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FB_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_2_intr_pol(&self) -> FB_2_INTR_POL_R {
        FB_2_INTR_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FB_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_3_intr_pol(&self) -> FB_3_INTR_POL_R {
        FB_3_INTR_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FB_0 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_0_intr_pol(&mut self) -> FB_0_INTR_POL_W {
        FB_0_INTR_POL_W { w: self }
    }
    #[doc = "Bit 1 - FB_1 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_1_intr_pol(&mut self) -> FB_1_INTR_POL_W {
        FB_1_INTR_POL_W { w: self }
    }
    #[doc = "Bit 2 - FB_2 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_2_intr_pol(&mut self) -> FB_2_INTR_POL_W {
        FB_2_INTR_POL_W { w: self }
    }
    #[doc = "Bit 3 - FB_3 interrupt polarity (depends on interrupt type): type = level => {0:low, 1:hi} --- type = edge => {0:fall, 1:rise}"]
    #[inline(always)]
    pub fn fb_3_intr_pol(&mut self) -> FB_3_INTR_POL_W {
        FB_3_INTR_POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt trigger polarities (will depend on the type of interrupt)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_intr_pol](index.html) module"]
pub struct FB_INTR_POL_SPEC;
impl crate::RegisterSpec for FB_INTR_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_intr_pol::R](R) reader structure"]
impl crate::Readable for FB_INTR_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_intr_pol::W](W) writer structure"]
impl crate::Writable for FB_INTR_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_INTR_POL to value 0"]
impl crate::Resettable for FB_INTR_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
