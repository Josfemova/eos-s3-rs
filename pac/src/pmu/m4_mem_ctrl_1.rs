#[doc = "Register `M4_MEM_CTRL_1` reader"]
pub struct R(crate::R<M4_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_MEM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_MEM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_MEM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_MEM_CTRL_1` writer"]
pub struct W(crate::W<M4_MEM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_MEM_CTRL_1_SPEC>;
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
impl From<crate::W<M4_MEM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_MEM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control SD pin of 32KB SRAM Instance 0 on M4 subsystem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_M4_SRAM_SD_0_A {
    #[doc = "0: Disable the shut down function of SRAM Macro."]
    DISABLE_SHUT_DOWN = 0,
    #[doc = "1: Enable the shut down function of SRAM Macro, Memory content will be lost."]
    ENABLE_SHUT_DOWN = 1,
}
impl From<CTRL_M4_SRAM_SD_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_M4_SRAM_SD_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRL_M4_SRAM_SD_0` reader - Control SD pin of 32KB SRAM Instance 0 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_0_R(crate::FieldReader<bool, CTRL_M4_SRAM_SD_0_A>);
impl CTRL_M4_SRAM_SD_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_M4_SRAM_SD_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_M4_SRAM_SD_0_A {
        match self.bits {
            false => CTRL_M4_SRAM_SD_0_A::DISABLE_SHUT_DOWN,
            true => CTRL_M4_SRAM_SD_0_A::ENABLE_SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_disable_shut_down(&self) -> bool {
        **self == CTRL_M4_SRAM_SD_0_A::DISABLE_SHUT_DOWN
    }
    #[doc = "Checks if the value of the field is `ENABLE_SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_enable_shut_down(&self) -> bool {
        **self == CTRL_M4_SRAM_SD_0_A::ENABLE_SHUT_DOWN
    }
}
impl core::ops::Deref for CTRL_M4_SRAM_SD_0_R {
    type Target = crate::FieldReader<bool, CTRL_M4_SRAM_SD_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_M4_SRAM_SD_0` writer - Control SD pin of 32KB SRAM Instance 0 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_0_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_0_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_1_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_1` reader - Control SD pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_1_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_1` writer - Control SD pin of 32KB SRAM Instance 1 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_1_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_1_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_2_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_2` reader - Control SD pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_2_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_2` writer - Control SD pin of 32KB SRAM Instance 2 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_2_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_2_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_3_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_3` reader - Control SD pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_3_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_3` writer - Control SD pin of 32KB SRAM Instance 3 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_3_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_3_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_4_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_4` reader - Control SD pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_4_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_4` writer - Control SD pin of 32KB SRAM Instance 4 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_4_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_4_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_5_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_5` reader - Control SD pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_5_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_5` writer - Control SD pin of 32KB SRAM Instance 5 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_5_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_5_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_6_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_6` reader - Control SD pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_6_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_6` writer - Control SD pin of 32KB SRAM Instance 6 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_6_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_6_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_7_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_7` reader - Control SD pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_7_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_7` writer - Control SD pin of 32KB SRAM Instance 7 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_7_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_7_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_8_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_8` reader - Control SD pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_8_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_8` writer - Control SD pin of 32KB SRAM Instance 8 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_8_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_8_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_9_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_9` reader - Control SD pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_9_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_9` writer - Control SD pin of 32KB SRAM Instance 9 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_9_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_9_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_9_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_10_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_10` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_10_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_10` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_10_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_10_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_11_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_11` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_11_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_11` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_11_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_11_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_12_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_12` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_12_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_12` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_12_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_12_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_13_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_13` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_13_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_13` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_13_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_13_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_14_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_14` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_14_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_14` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_14_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_14_A::ENABLE_SHUT_DOWN)
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
#[doc = "Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_15_A = CTRL_M4_SRAM_SD_0_A;
#[doc = "Field `CTRL_M4_SRAM_SD_15` reader - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub type CTRL_M4_SRAM_SD_15_R = CTRL_M4_SRAM_SD_0_R;
#[doc = "Field `CTRL_M4_SRAM_SD_15` writer - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
pub struct CTRL_M4_SRAM_SD_15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_M4_SRAM_SD_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_M4_SRAM_SD_15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the shut down function of SRAM Macro."]
    #[inline(always)]
    pub fn disable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_15_A::DISABLE_SHUT_DOWN)
    }
    #[doc = "Enable the shut down function of SRAM Macro, Memory content will be lost."]
    #[inline(always)]
    pub fn enable_shut_down(self) -> &'a mut W {
        self.variant(CTRL_M4_SRAM_SD_15_A::ENABLE_SHUT_DOWN)
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
    #[doc = "Bit 0 - Control SD pin of 32KB SRAM Instance 0 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_0(&self) -> CTRL_M4_SRAM_SD_0_R {
        CTRL_M4_SRAM_SD_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control SD pin of 32KB SRAM Instance 1 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_1(&self) -> CTRL_M4_SRAM_SD_1_R {
        CTRL_M4_SRAM_SD_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control SD pin of 32KB SRAM Instance 2 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_2(&self) -> CTRL_M4_SRAM_SD_2_R {
        CTRL_M4_SRAM_SD_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control SD pin of 32KB SRAM Instance 3 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_3(&self) -> CTRL_M4_SRAM_SD_3_R {
        CTRL_M4_SRAM_SD_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control SD pin of 32KB SRAM Instance 4 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_4(&self) -> CTRL_M4_SRAM_SD_4_R {
        CTRL_M4_SRAM_SD_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control SD pin of 32KB SRAM Instance 5 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_5(&self) -> CTRL_M4_SRAM_SD_5_R {
        CTRL_M4_SRAM_SD_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control SD pin of 32KB SRAM Instance 6 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_6(&self) -> CTRL_M4_SRAM_SD_6_R {
        CTRL_M4_SRAM_SD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control SD pin of 32KB SRAM Instance 7 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_7(&self) -> CTRL_M4_SRAM_SD_7_R {
        CTRL_M4_SRAM_SD_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Control SD pin of 32KB SRAM Instance 8 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_8(&self) -> CTRL_M4_SRAM_SD_8_R {
        CTRL_M4_SRAM_SD_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Control SD pin of 32KB SRAM Instance 9 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_9(&self) -> CTRL_M4_SRAM_SD_9_R {
        CTRL_M4_SRAM_SD_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_10(&self) -> CTRL_M4_SRAM_SD_10_R {
        CTRL_M4_SRAM_SD_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_11(&self) -> CTRL_M4_SRAM_SD_11_R {
        CTRL_M4_SRAM_SD_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_12(&self) -> CTRL_M4_SRAM_SD_12_R {
        CTRL_M4_SRAM_SD_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_13(&self) -> CTRL_M4_SRAM_SD_13_R {
        CTRL_M4_SRAM_SD_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_14(&self) -> CTRL_M4_SRAM_SD_14_R {
        CTRL_M4_SRAM_SD_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_15(&self) -> CTRL_M4_SRAM_SD_15_R {
        CTRL_M4_SRAM_SD_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control SD pin of 32KB SRAM Instance 0 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_0(&mut self) -> CTRL_M4_SRAM_SD_0_W {
        CTRL_M4_SRAM_SD_0_W { w: self }
    }
    #[doc = "Bit 1 - Control SD pin of 32KB SRAM Instance 1 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_1(&mut self) -> CTRL_M4_SRAM_SD_1_W {
        CTRL_M4_SRAM_SD_1_W { w: self }
    }
    #[doc = "Bit 2 - Control SD pin of 32KB SRAM Instance 2 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_2(&mut self) -> CTRL_M4_SRAM_SD_2_W {
        CTRL_M4_SRAM_SD_2_W { w: self }
    }
    #[doc = "Bit 3 - Control SD pin of 32KB SRAM Instance 3 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_3(&mut self) -> CTRL_M4_SRAM_SD_3_W {
        CTRL_M4_SRAM_SD_3_W { w: self }
    }
    #[doc = "Bit 4 - Control SD pin of 32KB SRAM Instance 4 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_4(&mut self) -> CTRL_M4_SRAM_SD_4_W {
        CTRL_M4_SRAM_SD_4_W { w: self }
    }
    #[doc = "Bit 5 - Control SD pin of 32KB SRAM Instance 5 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_5(&mut self) -> CTRL_M4_SRAM_SD_5_W {
        CTRL_M4_SRAM_SD_5_W { w: self }
    }
    #[doc = "Bit 6 - Control SD pin of 32KB SRAM Instance 6 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_6(&mut self) -> CTRL_M4_SRAM_SD_6_W {
        CTRL_M4_SRAM_SD_6_W { w: self }
    }
    #[doc = "Bit 7 - Control SD pin of 32KB SRAM Instance 7 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_7(&mut self) -> CTRL_M4_SRAM_SD_7_W {
        CTRL_M4_SRAM_SD_7_W { w: self }
    }
    #[doc = "Bit 8 - Control SD pin of 32KB SRAM Instance 8 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_8(&mut self) -> CTRL_M4_SRAM_SD_8_W {
        CTRL_M4_SRAM_SD_8_W { w: self }
    }
    #[doc = "Bit 9 - Control SD pin of 32KB SRAM Instance 9 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_9(&mut self) -> CTRL_M4_SRAM_SD_9_W {
        CTRL_M4_SRAM_SD_9_W { w: self }
    }
    #[doc = "Bit 10 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_10(&mut self) -> CTRL_M4_SRAM_SD_10_W {
        CTRL_M4_SRAM_SD_10_W { w: self }
    }
    #[doc = "Bit 11 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_11(&mut self) -> CTRL_M4_SRAM_SD_11_W {
        CTRL_M4_SRAM_SD_11_W { w: self }
    }
    #[doc = "Bit 12 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_12(&mut self) -> CTRL_M4_SRAM_SD_12_W {
        CTRL_M4_SRAM_SD_12_W { w: self }
    }
    #[doc = "Bit 13 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_13(&mut self) -> CTRL_M4_SRAM_SD_13_W {
        CTRL_M4_SRAM_SD_13_W { w: self }
    }
    #[doc = "Bit 14 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_14(&mut self) -> CTRL_M4_SRAM_SD_14_W {
        CTRL_M4_SRAM_SD_14_W { w: self }
    }
    #[doc = "Bit 15 - Control SD pin of 32KB SRAM Instance 10 on M4 subsystem"]
    #[inline(always)]
    pub fn ctrl_m4_sram_sd_15(&mut self) -> CTRL_M4_SRAM_SD_15_W {
        CTRL_M4_SRAM_SD_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Shutdown pin for various instances of SRAM on the M4 subsystem. For each instance: 1'b1 : Enable the Shutdown funciton of SRAM Macro, Memory content will be lost. While M4 access the memory in Shutdown mode, the HW will clear the corresponding bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_mem_ctrl_1](index.html) module"]
pub struct M4_MEM_CTRL_1_SPEC;
impl crate::RegisterSpec for M4_MEM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_mem_ctrl_1::R](R) reader structure"]
impl crate::Readable for M4_MEM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_mem_ctrl_1::W](W) writer structure"]
impl crate::Writable for M4_MEM_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_MEM_CTRL_1 to value 0"]
impl crate::Resettable for M4_MEM_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
