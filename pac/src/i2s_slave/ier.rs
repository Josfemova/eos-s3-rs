#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DW_apb_i2s_enable. A disable on this bit overrides any other block or channel enables and flushes all FIFOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN_A {
    #[doc = "0: Enable DW_APB_I2S."]
    DISABLE = 0,
    #[doc = "1: Disable DW_APB_I2S."]
    ENABLE = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEN` reader - DW_apb_i2s_enable. A disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub struct IEN_R(crate::FieldReader<bool, IEN_A>);
impl IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::DISABLE,
            true => IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IEN_A::ENABLE
    }
}
impl core::ops::Deref for IEN_R {
    type Target = crate::FieldReader<bool, IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEN` writer - DW_apb_i2s_enable. A disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable DW_APB_I2S."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN_A::DISABLE)
    }
    #[doc = "Disable DW_APB_I2S."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN_A::ENABLE)
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
    #[doc = "Bit 0 - DW_apb_i2s_enable. A disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DW_apb_i2s_enable. A disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
