#[doc = "Register `LDO_30_CTRL_0` reader"]
pub struct R(crate::R<LDO_30_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_30_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_30_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_30_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_30_CTRL_0` writer"]
pub struct W(crate::W<LDO_30_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_30_CTRL_0_SPEC>;
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
impl From<crate::W<LDO_30_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_30_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Used to disable LDO_30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_A {
    #[doc = "0: Enable LDO_30 output"]
    ENABLE = 0,
    #[doc = "1: Disable LDO_30 output"]
    DISABLE = 1,
}
impl From<DIS_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS` reader - Used to disable LDO_30"]
pub struct DIS_R(crate::FieldReader<bool, DIS_A>);
impl DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_A {
        match self.bits {
            false => DIS_A::ENABLE,
            true => DIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DIS_A::DISABLE
    }
}
impl core::ops::Deref for DIS_R {
    type Target = crate::FieldReader<bool, DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS` writer - Used to disable LDO_30"]
pub struct DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable LDO_30 output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIS_A::ENABLE)
    }
    #[doc = "Disable LDO_30 output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIS_A::DISABLE)
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
#[doc = "Used to disable the power good comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISPG_A {
    #[doc = "0: Enable LDO_30 power good comparator"]
    ENABLE = 0,
    #[doc = "1: Disable LDO_30 power good comparator"]
    DISABLE = 1,
}
impl From<DISPG_A> for bool {
    #[inline(always)]
    fn from(variant: DISPG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISPG` reader - Used to disable the power good comparator"]
pub struct DISPG_R(crate::FieldReader<bool, DISPG_A>);
impl DISPG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISPG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISPG_A {
        match self.bits {
            false => DISPG_A::ENABLE,
            true => DISPG_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DISPG_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DISPG_A::DISABLE
    }
}
impl core::ops::Deref for DISPG_R {
    type Target = crate::FieldReader<bool, DISPG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISPG` writer - Used to disable the power good comparator"]
pub struct DISPG_W<'a> {
    w: &'a mut W,
}
impl<'a> DISPG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISPG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable LDO_30 power good comparator"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISPG_A::ENABLE)
    }
    #[doc = "Disable LDO_30 power good comparator"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISPG_A::DISABLE)
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
#[doc = "Configures the control for maximum expected current imax current (mA)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMAX_A {
    #[doc = "0: Configure the maximum current for LDO_30 as 30mA"]
    MAX_30MA_CURRENT = 0,
    #[doc = "1: Configure the maximum current for LDO_30 as 18mA"]
    MAX_18MA_CURRENT = 1,
    #[doc = "2: Configure the maximum current for LDO_30 as 12mA"]
    MAX_12MA_CURRENT = 2,
    #[doc = "3: Configure the maximum current for LDO_30 as 7.2mA (default)"]
    MAX_7_2MA_CURRENT = 3,
    #[doc = "4: Configure the maximum current for LDO_30 as 4.8mA"]
    MAX_4_8MA_CURRENT = 4,
    #[doc = "5: Configure the maximum current for LDO_30 as 2.4mA"]
    MAX_2_4MA_CURRENT = 5,
    #[doc = "6: Configure the maximum current for LDO_30 as 1.2mA"]
    MAX_1_2MA_CURRENT = 6,
    #[doc = "7: Configure the maximum current for LDO_30 as 0.6mA"]
    MAX_0_6MA_CURRENT = 7,
}
impl From<IMAX_A> for u8 {
    #[inline(always)]
    fn from(variant: IMAX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IMAX` reader - Configures the control for maximum expected current imax current (mA)"]
pub struct IMAX_R(crate::FieldReader<u8, IMAX_A>);
impl IMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMAX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAX_A {
        match self.bits {
            0 => IMAX_A::MAX_30MA_CURRENT,
            1 => IMAX_A::MAX_18MA_CURRENT,
            2 => IMAX_A::MAX_12MA_CURRENT,
            3 => IMAX_A::MAX_7_2MA_CURRENT,
            4 => IMAX_A::MAX_4_8MA_CURRENT,
            5 => IMAX_A::MAX_2_4MA_CURRENT,
            6 => IMAX_A::MAX_1_2MA_CURRENT,
            7 => IMAX_A::MAX_0_6MA_CURRENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAX_30MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_30m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_30MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_18MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_18m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_18MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_12MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_12m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_12MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_7_2MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_7_2m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_7_2MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_4_8MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_4_8m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_4_8MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_2_4MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_2_4m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_2_4MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_1_2MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_1_2m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_1_2MA_CURRENT
    }
    #[doc = "Checks if the value of the field is `MAX_0_6MA_CURRENT`"]
    #[inline(always)]
    pub fn is_max_0_6m_a_current(&self) -> bool {
        **self == IMAX_A::MAX_0_6MA_CURRENT
    }
}
impl core::ops::Deref for IMAX_R {
    type Target = crate::FieldReader<u8, IMAX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMAX` writer - Configures the control for maximum expected current imax current (mA)"]
pub struct IMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMAX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure the maximum current for LDO_30 as 30mA"]
    #[inline(always)]
    pub fn max_30m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_30MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 18mA"]
    #[inline(always)]
    pub fn max_18m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_18MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 12mA"]
    #[inline(always)]
    pub fn max_12m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_12MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 7.2mA (default)"]
    #[inline(always)]
    pub fn max_7_2m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_7_2MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 4.8mA"]
    #[inline(always)]
    pub fn max_4_8m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_4_8MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 2.4mA"]
    #[inline(always)]
    pub fn max_2_4m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_2_4MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 1.2mA"]
    #[inline(always)]
    pub fn max_1_2m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_1_2MA_CURRENT)
    }
    #[doc = "Configure the maximum current for LDO_30 as 0.6mA"]
    #[inline(always)]
    pub fn max_0_6m_a_current(self) -> &'a mut W {
        self.variant(IMAX_A::MAX_0_6MA_CURRENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Output voltage programming. Note: Please keep LDO_30_DI and LDO_50_DI to be equal values for proper operation and lower power consumption.\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DI_A {
    #[doc = "0: Configures the LDO_30 output voltage as 0.75v"]
    AS_0_75V_OUTPUT = 0,
    #[doc = "1: Configures the LDO_30 output voltage as 0.77v"]
    AS_0_77V_OUTPUT = 1,
    #[doc = "2: Configures the LDO_30 output voltage as 0.79v"]
    AS_0_79V_OUTPUT = 2,
    #[doc = "3: Configures the LDO_30 output voltage as 0.81v"]
    AS_0_81V_OUTPUT = 3,
    #[doc = "4: Configures the LDO_30 output voltage as 0.83v"]
    AS_0_83V_OUTPUT = 4,
    #[doc = "5: Configures the LDO_30 output voltage as 0.85v"]
    AS_0_85V_OUTPUT = 5,
    #[doc = "6: Configures the LDO_30 output voltage as 0.87v"]
    AS_0_87V_OUTPUT = 6,
    #[doc = "7: Configures the LDO_30 output voltage as 0.89v"]
    AS_0_89V_OUTPUT = 7,
    #[doc = "8: Configures the LDO_30 output voltage as 0.91v"]
    AS_0_91V_OUTPUT = 8,
    #[doc = "9: Configures the LDO_30 output voltage as 0.93v"]
    AS_0_93V_OUTPUT = 9,
    #[doc = "10: Configures the LDO_30 output voltage as 0.95v"]
    AS_0_95V_OUTPUT = 10,
    #[doc = "11: Configures the LDO_30 output voltage as 0.97v"]
    AS_0_97V_OUTPUT = 11,
    #[doc = "12: Configures the LDO_30 output voltage as 0.99v"]
    AS_0_99V_OUTPUT = 12,
    #[doc = "13: Configures the LDO_30 output voltage as 1.01v"]
    AS_1_01V_OUTPUT = 13,
    #[doc = "14: Configures the LDO_30 output voltage as 1.03v"]
    AS_1_03V_OUTPUT = 14,
    #[doc = "15: Configures the LDO_30 output voltage as 1.05v"]
    AS_1_05V_OUTPUT = 15,
    #[doc = "16: Configures the LDO_30 output voltage as 1.07v"]
    AS_1_07V_OUTPUT = 16,
    #[doc = "17: Configures the LDO_30 output voltage as 1.09v (default)"]
    AS_1_09V_OUTPUT = 17,
    #[doc = "18: Configures the LDO_30 output voltage as 1.11v"]
    AS_1_11V_OUTPUT = 18,
    #[doc = "19: Configures the LDO_30 output voltage as 1.13v"]
    AS_1_13V_OUTPUT = 19,
    #[doc = "20: Configures the LDO_30 output voltage as 1.15v"]
    AS_1_15V_OUTPUT = 20,
    #[doc = "21: Configures the LDO_30 output voltage as 1.17v"]
    AS_1_17V_OUTPUT = 21,
    #[doc = "22: Configures the LDO_30 output voltage as 1.19v"]
    AS_1_19V_OUTPUT = 22,
    #[doc = "23: Configures the LDO_30 output voltage as 1.21v"]
    AS_1_21V_OUTPUT = 23,
}
impl From<DI_A> for u8 {
    #[inline(always)]
    fn from(variant: DI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DI` reader - Output voltage programming. Note: Please keep LDO_30_DI and LDO_50_DI to be equal values for proper operation and lower power consumption."]
pub struct DI_R(crate::FieldReader<u8, DI_A>);
impl DI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DI_A> {
        match self.bits {
            0 => Some(DI_A::AS_0_75V_OUTPUT),
            1 => Some(DI_A::AS_0_77V_OUTPUT),
            2 => Some(DI_A::AS_0_79V_OUTPUT),
            3 => Some(DI_A::AS_0_81V_OUTPUT),
            4 => Some(DI_A::AS_0_83V_OUTPUT),
            5 => Some(DI_A::AS_0_85V_OUTPUT),
            6 => Some(DI_A::AS_0_87V_OUTPUT),
            7 => Some(DI_A::AS_0_89V_OUTPUT),
            8 => Some(DI_A::AS_0_91V_OUTPUT),
            9 => Some(DI_A::AS_0_93V_OUTPUT),
            10 => Some(DI_A::AS_0_95V_OUTPUT),
            11 => Some(DI_A::AS_0_97V_OUTPUT),
            12 => Some(DI_A::AS_0_99V_OUTPUT),
            13 => Some(DI_A::AS_1_01V_OUTPUT),
            14 => Some(DI_A::AS_1_03V_OUTPUT),
            15 => Some(DI_A::AS_1_05V_OUTPUT),
            16 => Some(DI_A::AS_1_07V_OUTPUT),
            17 => Some(DI_A::AS_1_09V_OUTPUT),
            18 => Some(DI_A::AS_1_11V_OUTPUT),
            19 => Some(DI_A::AS_1_13V_OUTPUT),
            20 => Some(DI_A::AS_1_15V_OUTPUT),
            21 => Some(DI_A::AS_1_17V_OUTPUT),
            22 => Some(DI_A::AS_1_19V_OUTPUT),
            23 => Some(DI_A::AS_1_21V_OUTPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AS_0_75V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_75v_output(&self) -> bool {
        **self == DI_A::AS_0_75V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_77V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_77v_output(&self) -> bool {
        **self == DI_A::AS_0_77V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_79V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_79v_output(&self) -> bool {
        **self == DI_A::AS_0_79V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_81V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_81v_output(&self) -> bool {
        **self == DI_A::AS_0_81V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_83V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_83v_output(&self) -> bool {
        **self == DI_A::AS_0_83V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_85V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_85v_output(&self) -> bool {
        **self == DI_A::AS_0_85V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_87V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_87v_output(&self) -> bool {
        **self == DI_A::AS_0_87V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_89V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_89v_output(&self) -> bool {
        **self == DI_A::AS_0_89V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_91V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_91v_output(&self) -> bool {
        **self == DI_A::AS_0_91V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_93V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_93v_output(&self) -> bool {
        **self == DI_A::AS_0_93V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_95V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_95v_output(&self) -> bool {
        **self == DI_A::AS_0_95V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_97V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_97v_output(&self) -> bool {
        **self == DI_A::AS_0_97V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_0_99V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_0_99v_output(&self) -> bool {
        **self == DI_A::AS_0_99V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_01V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_01v_output(&self) -> bool {
        **self == DI_A::AS_1_01V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_03V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_03v_output(&self) -> bool {
        **self == DI_A::AS_1_03V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_05V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_05v_output(&self) -> bool {
        **self == DI_A::AS_1_05V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_07V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_07v_output(&self) -> bool {
        **self == DI_A::AS_1_07V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_09V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_09v_output(&self) -> bool {
        **self == DI_A::AS_1_09V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_11V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_11v_output(&self) -> bool {
        **self == DI_A::AS_1_11V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_13V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_13v_output(&self) -> bool {
        **self == DI_A::AS_1_13V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_15V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_15v_output(&self) -> bool {
        **self == DI_A::AS_1_15V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_17V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_17v_output(&self) -> bool {
        **self == DI_A::AS_1_17V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_19V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_19v_output(&self) -> bool {
        **self == DI_A::AS_1_19V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AS_1_21V_OUTPUT`"]
    #[inline(always)]
    pub fn is_as_1_21v_output(&self) -> bool {
        **self == DI_A::AS_1_21V_OUTPUT
    }
}
impl core::ops::Deref for DI_R {
    type Target = crate::FieldReader<u8, DI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DI` writer - Output voltage programming. Note: Please keep LDO_30_DI and LDO_50_DI to be equal values for proper operation and lower power consumption."]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configures the LDO_30 output voltage as 0.75v"]
    #[inline(always)]
    pub fn as_0_75v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_75V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.77v"]
    #[inline(always)]
    pub fn as_0_77v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_77V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.79v"]
    #[inline(always)]
    pub fn as_0_79v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_79V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.81v"]
    #[inline(always)]
    pub fn as_0_81v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_81V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.83v"]
    #[inline(always)]
    pub fn as_0_83v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_83V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.85v"]
    #[inline(always)]
    pub fn as_0_85v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_85V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.87v"]
    #[inline(always)]
    pub fn as_0_87v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_87V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.89v"]
    #[inline(always)]
    pub fn as_0_89v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_89V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.91v"]
    #[inline(always)]
    pub fn as_0_91v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_91V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.93v"]
    #[inline(always)]
    pub fn as_0_93v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_93V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.95v"]
    #[inline(always)]
    pub fn as_0_95v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_95V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.97v"]
    #[inline(always)]
    pub fn as_0_97v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_97V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 0.99v"]
    #[inline(always)]
    pub fn as_0_99v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_0_99V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.01v"]
    #[inline(always)]
    pub fn as_1_01v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_01V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.03v"]
    #[inline(always)]
    pub fn as_1_03v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_03V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.05v"]
    #[inline(always)]
    pub fn as_1_05v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_05V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.07v"]
    #[inline(always)]
    pub fn as_1_07v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_07V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.09v (default)"]
    #[inline(always)]
    pub fn as_1_09v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_09V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.11v"]
    #[inline(always)]
    pub fn as_1_11v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_11V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.13v"]
    #[inline(always)]
    pub fn as_1_13v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_13V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.15v"]
    #[inline(always)]
    pub fn as_1_15v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_15V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.17v"]
    #[inline(always)]
    pub fn as_1_17v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_17V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.19v"]
    #[inline(always)]
    pub fn as_1_19v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_19V_OUTPUT)
    }
    #[doc = "Configures the LDO_30 output voltage as 1.21v"]
    #[inline(always)]
    pub fn as_1_21v_output(self) -> &'a mut W {
        self.variant(DI_A::AS_1_21V_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Used to disable LDO_30"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Used to disable the power good comparator"]
    #[inline(always)]
    pub fn dispg(&self) -> DISPG_R {
        DISPG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Configures the control for maximum expected current imax current (mA)"]
    #[inline(always)]
    pub fn imax(&self) -> IMAX_R {
        IMAX_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:9 - Output voltage programming. Note: Please keep LDO_30_DI and LDO_50_DI to be equal values for proper operation and lower power consumption."]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Used to disable LDO_30"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W {
        DIS_W { w: self }
    }
    #[doc = "Bit 1 - Used to disable the power good comparator"]
    #[inline(always)]
    pub fn dispg(&mut self) -> DISPG_W {
        DISPG_W { w: self }
    }
    #[doc = "Bits 2:4 - Configures the control for maximum expected current imax current (mA)"]
    #[inline(always)]
    pub fn imax(&mut self) -> IMAX_W {
        IMAX_W { w: self }
    }
    #[doc = "Bits 5:9 - Output voltage programming. Note: Please keep LDO_30_DI and LDO_50_DI to be equal values for proper operation and lower power consumption."]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO_30 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_30_ctrl_0](index.html) module"]
pub struct LDO_30_CTRL_0_SPEC;
impl crate::RegisterSpec for LDO_30_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_30_ctrl_0::R](R) reader structure"]
impl crate::Readable for LDO_30_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_30_ctrl_0::W](W) writer structure"]
impl crate::Writable for LDO_30_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO_30_CTRL_0 to value 0x022c"]
impl crate::Resettable for LDO_30_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x022c
    }
}
