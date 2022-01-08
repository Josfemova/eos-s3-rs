#[doc = "Register `M4_MEM_CTRL_0` reader"]
pub struct R(crate::R<M4_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_MEM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_MEM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_MEM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_MEM_CTRL_0` writer"]
pub struct W(crate::W<M4_MEM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_MEM_CTRL_0_SPEC>;
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
impl From<crate::W<M4_MEM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_MEM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 0 on M4 subsystem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_M4_SRAM_DS_0_A {
    #[doc = "0: Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    DISABLE_DEEP_SLEEP = 0,
    #[doc = "1: Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    ENABLE_DEEP_SLEEP = 1,
}
impl From<CTRL_M4_SRAM_DS_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_M4_SRAM_DS_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRL_M4_SRAM_DS_0` reader - Control DS pin of 32KB SRAM Instance 0 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_0_R(crate::FieldReader<bool, CTRL_M4_SRAM_DS_0_A>);
impl CTRL_M4_SRAM_DS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_M4_SRAM_DS_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_M4_SRAM_DS_0_A {
        match self.bits {
            false => CTRL_M4_SRAM_DS_0_A::DISABLE_DEEP_SLEEP,
            true => CTRL_M4_SRAM_DS_0_A::ENABLE_DEEP_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_disable_deep_sleep(&self) -> bool {
        **self == CTRL_M4_SRAM_DS_0_A::DISABLE_DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLE_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_enable_deep_sleep(&self) -> bool {
        **self == CTRL_M4_SRAM_DS_0_A::ENABLE_DEEP_SLEEP
    }
}
impl core::ops::Deref for CTRL_M4_SRAM_DS_0_R {
    type Target = crate::FieldReader<bool, CTRL_M4_SRAM_DS_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_M4_SRAM_DS_0` writer - Control DS pin of 32KB SRAM Instance 0 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_0_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_0_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Control DS pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_1_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_1` reader - Control DS pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_1_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_1` writer - Control DS pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_1_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_1_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Control DS pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_2_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_2` reader - Control DS pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_2_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_2` writer - Control DS pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_2_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_2_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Control DS pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_3_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_3` reader - Control DS pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_3_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_3` writer - Control DS pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_3_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_3_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Control DS pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_4_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_4` reader - Control DS pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_4_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_4` writer - Control DS pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_4_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_4_A::ENABLE_DEEP_SLEEP)
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
#[doc = "Control DS pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_5_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_5` reader - Control DS pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_5_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_5` writer - Control DS pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_5_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_5_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_6_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_6` reader - Control DS pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_6_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_6` writer - Control DS pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_6_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_6_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_7_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_7` reader - Control DS pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_7_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_7` writer - Control DS pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_7_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_7_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_8_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_8` reader - Control DS pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_8_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_8` writer - Control DS pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_8_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_8_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_9_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_9` reader - Control DS pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_9_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_9` writer - Control DS pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_9_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_9_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_9_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_10_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_10` reader - Control DS pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_10_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_10` writer - Control DS pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_10_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_10_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 11 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_11_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_11` reader - Control DS pin of 32KB SRAM Instance 11 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_11_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_11` writer - Control DS pin of 32KB SRAM Instance 11 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_11_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_11_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 12 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_12_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_12` reader - Control DS pin of 32KB SRAM Instance 12 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_12_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_12` writer - Control DS pin of 32KB SRAM Instance 12 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_12_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_12_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 13 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_13_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_13` reader - Control DS pin of 32KB SRAM Instance 13 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_13_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_13` writer - Control DS pin of 32KB SRAM Instance 13 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_13_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_13_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 14 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_14_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_14` reader - Control DS pin of 32KB SRAM Instance 14 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_14_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_14` writer - Control DS pin of 32KB SRAM Instance 14 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_14_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_14_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Control DS pin of 32KB SRAM Instance 15 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_15_A = CTRL_M4_SRAM_DS_0_A;
#[doc = "Field `CTRL_M4_SRAM_DS_15` reader - Control DS pin of 32KB SRAM Instance 15 on M4 subsystem"]
pub type CTRL_M4_SRAM_DS_15_R = CTRL_M4_SRAM_DS_0_R;
#[doc = "Field `CTRL_M4_SRAM_DS_15` writer - Control DS pin of 32KB SRAM Instance 15 on M4 subsystem"]
pub struct CTRL_M4_SRAM_DS_15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_DS_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_DS_15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn disable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_15_A::DISABLE_DEEP_SLEEP)
    }
    #[doc = "Enable the Deep Sleep function of SRAM Macro, Memory content will be kept."]
    #[inline(always)]
    pub fn enable_deep_sleep(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_DS_15_A::ENABLE_DEEP_SLEEP)
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
            (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control DS pin of 32KB SRAM Instance 0 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_0(&self) -> CTRL_M4_SRAM_DS_0_R {
        CTRL_M4_SRAM_DS_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control DS pin of 32KB SRAM Instance 1 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_1(&self) -> CTRL_M4_SRAM_DS_1_R {
        CTRL_M4_SRAM_DS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control DS pin of 32KB SRAM Instance 2 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_2(&self) -> CTRL_M4_SRAM_DS_2_R {
        CTRL_M4_SRAM_DS_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control DS pin of 32KB SRAM Instance 3 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_3(&self) -> CTRL_M4_SRAM_DS_3_R {
        CTRL_M4_SRAM_DS_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control DS pin of 32KB SRAM Instance 4 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_4(&self) -> CTRL_M4_SRAM_DS_4_R {
        CTRL_M4_SRAM_DS_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control DS pin of 32KB SRAM Instance 5 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_5(&self) -> CTRL_M4_SRAM_DS_5_R {
        CTRL_M4_SRAM_DS_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control DS pin of 32KB SRAM Instance 6 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_6(&self) -> CTRL_M4_SRAM_DS_6_R {
        CTRL_M4_SRAM_DS_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control DS pin of 32KB SRAM Instance 7 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_7(&self) -> CTRL_M4_SRAM_DS_7_R {
        CTRL_M4_SRAM_DS_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Control DS pin of 32KB SRAM Instance 8 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_8(&self) -> CTRL_M4_SRAM_DS_8_R {
        CTRL_M4_SRAM_DS_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Control DS pin of 32KB SRAM Instance 9 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_9(&self) -> CTRL_M4_SRAM_DS_9_R {
        CTRL_M4_SRAM_DS_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Control DS pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_10(&self) -> CTRL_M4_SRAM_DS_10_R {
        CTRL_M4_SRAM_DS_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Control DS pin of 32KB SRAM Instance 11 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_11(&self) -> CTRL_M4_SRAM_DS_11_R {
        CTRL_M4_SRAM_DS_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Control DS pin of 32KB SRAM Instance 12 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_12(&self) -> CTRL_M4_SRAM_DS_12_R {
        CTRL_M4_SRAM_DS_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Control DS pin of 32KB SRAM Instance 13 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_13(&self) -> CTRL_M4_SRAM_DS_13_R {
        CTRL_M4_SRAM_DS_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control DS pin of 32KB SRAM Instance 14 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_14(&self) -> CTRL_M4_SRAM_DS_14_R {
        CTRL_M4_SRAM_DS_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Control DS pin of 32KB SRAM Instance 15 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_15(&self) -> CTRL_M4_SRAM_DS_15_R {
        CTRL_M4_SRAM_DS_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control DS pin of 32KB SRAM Instance 0 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_0(&mut self) -> CTRL_M4_SRAM_DS_0_W {
        CTRL_M4_SRAM_DS_0_W { w: self }
    }
    #[doc = "Bit 1 - Control DS pin of 32KB SRAM Instance 1 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_1(&mut self) -> CTRL_M4_SRAM_DS_1_W {
        CTRL_M4_SRAM_DS_1_W { w: self }
    }
    #[doc = "Bit 2 - Control DS pin of 32KB SRAM Instance 2 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_2(&mut self) -> CTRL_M4_SRAM_DS_2_W {
        CTRL_M4_SRAM_DS_2_W { w: self }
    }
    #[doc = "Bit 3 - Control DS pin of 32KB SRAM Instance 3 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_3(&mut self) -> CTRL_M4_SRAM_DS_3_W {
        CTRL_M4_SRAM_DS_3_W { w: self }
    }
    #[doc = "Bit 4 - Control DS pin of 32KB SRAM Instance 4 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_4(&mut self) -> CTRL_M4_SRAM_DS_4_W {
        CTRL_M4_SRAM_DS_4_W { w: self }
    }
    #[doc = "Bit 5 - Control DS pin of 32KB SRAM Instance 5 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_5(&mut self) -> CTRL_M4_SRAM_DS_5_W {
        CTRL_M4_SRAM_DS_5_W { w: self }
    }
    #[doc = "Bit 6 - Control DS pin of 32KB SRAM Instance 6 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_6(&mut self) -> CTRL_M4_SRAM_DS_6_W {
        CTRL_M4_SRAM_DS_6_W { w: self }
    }
    #[doc = "Bit 7 - Control DS pin of 32KB SRAM Instance 7 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_7(&mut self) -> CTRL_M4_SRAM_DS_7_W {
        CTRL_M4_SRAM_DS_7_W { w: self }
    }
    #[doc = "Bit 8 - Control DS pin of 32KB SRAM Instance 8 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_8(&mut self) -> CTRL_M4_SRAM_DS_8_W {
        CTRL_M4_SRAM_DS_8_W { w: self }
    }
    #[doc = "Bit 9 - Control DS pin of 32KB SRAM Instance 9 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_9(&mut self) -> CTRL_M4_SRAM_DS_9_W {
        CTRL_M4_SRAM_DS_9_W { w: self }
    }
    #[doc = "Bit 10 - Control DS pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_10(&mut self) -> CTRL_M4_SRAM_DS_10_W {
        CTRL_M4_SRAM_DS_10_W { w: self }
    }
    #[doc = "Bit 11 - Control DS pin of 32KB SRAM Instance 11 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_11(&mut self) -> CTRL_M4_SRAM_DS_11_W {
        CTRL_M4_SRAM_DS_11_W { w: self }
    }
    #[doc = "Bit 12 - Control DS pin of 32KB SRAM Instance 12 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_12(&mut self) -> CTRL_M4_SRAM_DS_12_W {
        CTRL_M4_SRAM_DS_12_W { w: self }
    }
    #[doc = "Bit 13 - Control DS pin of 32KB SRAM Instance 13 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_13(&mut self) -> CTRL_M4_SRAM_DS_13_W {
        CTRL_M4_SRAM_DS_13_W { w: self }
    }
    #[doc = "Bit 14 - Control DS pin of 32KB SRAM Instance 14 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_14(&mut self) -> CTRL_M4_SRAM_DS_14_W {
        CTRL_M4_SRAM_DS_14_W { w: self }
    }
    #[doc = "Bit 15 - Control DS pin of 32KB SRAM Instance 15 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_ds_15(&mut self) -> CTRL_M4_SRAM_DS_15_W {
        CTRL_M4_SRAM_DS_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control DS pins for different SRAM instances on the M4 subsystem. For each instance: 1'b1 : Enable the Deep Sleep funciton of SRAM Macro, Memory content will be kept. While M4 access the memory in Deep Sleep mode, the HW will clear the corresponding bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_mem_ctrl_0](index.html) module"]
pub struct M4_MEM_CTRL_0_SPEC;
impl crate::RegisterSpec for M4_MEM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_mem_ctrl_0::R](R) reader structure"]
impl crate::Readable for M4_MEM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_mem_ctrl_0::W](W) writer structure"]
impl crate::Writable for M4_MEM_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_MEM_CTRL_0 to value 0xfffe"]
impl crate::Resettable for M4_MEM_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfffe
    }
}
