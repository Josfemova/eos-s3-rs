#[doc = "Register `LOCK_KEY_CTRL` reader"]
pub struct R(crate::R<LOCK_KEY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_KEY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_KEY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_KEY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK_KEY_CTRL` writer"]
pub struct W(crate::W<LOCK_KEY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_KEY_CTRL_SPEC>;
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
impl From<crate::W<LOCK_KEY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_KEY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "0: lock disabled, write to register enabled, 1: lock enable, write to register disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_KEY_EN_A {
    #[doc = "0: Disable lock to allow writing the lock_key field"]
    LOCK_DISABLE = 0,
    #[doc = "1: Enable lock to prevent writing the lock_key field"]
    LOCK_ENABLE = 1,
}
impl From<LOCK_KEY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_KEY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_KEY_EN` reader - 0: lock disabled, write to register enabled, 1: lock enable, write to register disabled"]
pub struct LOCK_KEY_EN_R(crate::FieldReader<bool, LOCK_KEY_EN_A>);
impl LOCK_KEY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_KEY_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_KEY_EN_A {
        match self.bits {
            false => LOCK_KEY_EN_A::LOCK_DISABLE,
            true => LOCK_KEY_EN_A::LOCK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DISABLE`"]
    #[inline(always)]
    pub fn is_lock_disable(&self) -> bool {
        **self == LOCK_KEY_EN_A::LOCK_DISABLE
    }
    #[doc = "Checks if the value of the field is `LOCK_ENABLE`"]
    #[inline(always)]
    pub fn is_lock_enable(&self) -> bool {
        **self == LOCK_KEY_EN_A::LOCK_ENABLE
    }
}
impl core::ops::Deref for LOCK_KEY_EN_R {
    type Target = crate::FieldReader<bool, LOCK_KEY_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_KEY_EN` writer - 0: lock disabled, write to register enabled, 1: lock enable, write to register disabled"]
pub struct LOCK_KEY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_KEY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_KEY_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable lock to allow writing the lock_key field"]
    #[inline(always)]
    pub fn lock_disable(self) -> &'a mut W {
        self.variant(LOCK_KEY_EN_A::LOCK_DISABLE)
    }
    #[doc = "Enable lock to prevent writing the lock_key field"]
    #[inline(always)]
    pub fn lock_enable(self) -> &'a mut W {
        self.variant(LOCK_KEY_EN_A::LOCK_ENABLE)
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
#[doc = "Field `LOCK_KEY` writer - Enable write access to all below registers by writing 0x1ACCE551. Disable write access by writing any other value. M4 WDT Intr/reset clear - 0x4000_4830\\[4:3\\], M4 WDT Intr/reset enable AP - 0x4000_4834\\[4:3\\], Pad #43 (AP_INTR) - 0x40004CAC\\[12:0\\], M4 Low Power Configuration - 0x40004484\\[1:0\\], M4 WDT Clock Gate - 0x40004054\\[0\\]"]
pub struct LOCK_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1))
            | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: lock disabled, write to register enabled, 1: lock enable, write to register disabled"]
    #[inline(always)]
    pub fn lock_key_en(&self) -> LOCK_KEY_EN_R {
        LOCK_KEY_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: lock disabled, write to register enabled, 1: lock enable, write to register disabled"]
    #[inline(always)]
    pub fn lock_key_en(&mut self) -> LOCK_KEY_EN_W {
        LOCK_KEY_EN_W { w: self }
    }
    #[doc = "Bits 1:31 - Enable write access to all below registers by writing 0x1ACCE551. Disable write access by writing any other value. M4 WDT Intr/reset clear - 0x4000_4830\\[4:3\\], M4 WDT Intr/reset enable AP - 0x4000_4834\\[4:3\\], Pad #43 (AP_INTR) - 0x40004CAC\\[12:0\\], M4 Low Power Configuration - 0x40004484\\[1:0\\], M4 WDT Clock Gate - 0x40004054\\[0\\]"]
    #[inline(always)]
    pub fn lock_key(&mut self) -> LOCK_KEY_W {
        LOCK_KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control value and status of LOCK_KEY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_key_ctrl](index.html) module"]
pub struct LOCK_KEY_CTRL_SPEC;
impl crate::RegisterSpec for LOCK_KEY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock_key_ctrl::R](R) reader structure"]
impl crate::Readable for LOCK_KEY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock_key_ctrl::W](W) writer structure"]
impl crate::Writable for LOCK_KEY_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK_KEY_CTRL to value 0x1acc_e551"]
impl crate::Resettable for LOCK_KEY_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1acc_e551
    }
}
