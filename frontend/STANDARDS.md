# Sleep Quality Standards Documentation

Scientific standards and references used for sleep environment assessment.

---

## Overview

This document outlines the evidence-based standards used to evaluate sleep quality based on environmental factors. The ML model uses these standards to calculate sleep quality scores and classifications.

---

## Scoring System

**Overall Score Range:** 0-100 points

**Classification:**
- **Good Sleep:** 70-100 points (Optimal environmental conditions)
- **Poor Sleep:** 0-69 points (Suboptimal environmental conditions)

**Component Weights:**
- Temperature: 30% of total score
- Humidity: 25% of total score
- Sound Level: 25% of total score
- Motion: 20% of total score

---

## 1. Temperature Standards

### Optimal Range
**18-22°C (64-72°F)**

### Scientific Basis
Core body temperature naturally decreases during sleep as part of the circadian rhythm. A cool room temperature facilitates this natural thermoregulatory process, promoting deeper and more restful sleep.

### Scoring Criteria

| Temperature Range | Points | Quality |
|-------------------|--------|---------|
| 18-22°C | 100% | Optimal |
| 16-18°C or 22-24°C | 70% | Acceptable |
| 14-16°C or 24-26°C | 40% | Suboptimal |
| <14°C or >26°C | 10% | Poor |

### Research References

1. **National Sleep Foundation**
 - Title: "The Best Temperature for Sleep"
 - Link: https://www.sleepfoundation.org/bedroom-environment/best-temperature-for-sleep
 - Key Finding: Optimal bedroom temperature is between 60-67°F (15.5-19.5°C)

2. **Okamoto-Mizuno, K., & Mizuno, K. (2012)**
 - Title: "Effects of thermal environment on sleep and circadian rhythm"
 - Journal: Journal of Physiological Anthropology, 31(1), 14
 - DOI: 10.1186/1880-6805-31-14
 - Key Finding: Thermal environment significantly affects sleep stages and sleep quality

3. **Haskell, E. H., et al. (1981)**
 - Title: "The effects of high and low ambient temperatures on human sleep stages"
 - Journal: Electroencephalography and Clinical Neurophysiology, 51(5), 494-501
 - Key Finding: Deviations from thermoneutral zone reduce REM sleep and increase wakefulness

---

## 2. Humidity Standards

### Optimal Range
**40-60%**

### Scientific Basis
Proper humidity levels prevent respiratory tract dryness and promote comfortable breathing during sleep. Low humidity can cause nasal congestion and throat irritation, while high humidity promotes mold growth and dust mites.

### Scoring Criteria

| Humidity Range | Points | Quality |
|----------------|--------|---------|
| 40-60% | 100% | Optimal |
| 30-40% or 60-70% | 70% | Acceptable |
| 20-30% or 70-80% | 40% | Suboptimal |
| <20% or >80% | 10% | Poor |

### Research References

1. **American Academy of Sleep Medicine (AASM)**
 - Recommendation: Maintain bedroom humidity between 30-50%
 - Link: https://aasm.org/
 - Key Finding: Proper humidity improves sleep quality and reduces respiratory issues

2. **Arens, E., & Zhang, H. (2006)**
 - Title: "The skin's role in human thermoregulation and comfort"
 - Chapter in: Thermal and Moisture Transport in Fibrous Materials
 - Key Finding: Relative humidity affects thermal comfort and skin moisture

3. **Wolkoff, P., & Kjaergaard, S. K. (2007)**
 - Title: "The dichotomy of relative humidity on indoor air quality"
 - Journal: Environment International, 33(6), 850-857
 - DOI: 10.1016/j.envint.2007.04.004
 - Key Finding: 40-60% RH optimal for health and comfort

---

## 3. Sound Level Standards

### Optimal Range
**<40 dB (Quiet library level)**

### Scientific Basis
Noise pollution disrupts sleep architecture, particularly REM and deep sleep stages. Even low-level noise can cause arousals and sleep fragmentation without the sleeper's awareness.

### Scoring Criteria

| Sound Level | Points | Quality | Example |
|-------------|--------|---------|---------|
| <30 dB | 100% | Very Quiet | Whisper |
| 30-40 dB | 90% | Quiet | Library |
| 40-55 dB | 60% | Moderate | Quiet conversation |
| 55-70 dB | 30% | Disruptive | Office environment |
| >70 dB | 10% | Very Disruptive | Traffic noise |

### Research References

1. **World Health Organization (2018)**
 - Title: "Environmental Noise Guidelines for the European Region"
 - Link: https://www.who.int/publications/i/item/9789289053563
 - Key Finding: Night noise levels above 40 dB negatively impact sleep quality

2. **Basner, M., et al. (2014)**
 - Title: "Auditory and non-auditory effects of noise on health"
 - Journal: The Lancet, 383(9925), 1325-1332
 - DOI: 10.1016/S0140-6736(13)61613-X
 - Key Finding: Noise-induced sleep fragmentation has cumulative health effects

3. **Hume, K. I., et al. (2012)**
 - Title: "Effects of environmental noise on sleep"
 - Journal: Noise and Health, 14(61), 297-302
 - DOI: 10.4103/1463-1741.104899
 - Key Finding: Even low-level noise (30-40 dB) can affect sleep quality

4. **Muzet, A. (2007)**
 - Title: "Environmental noise, sleep and health"
 - Journal: Sleep Medicine Reviews, 11(2), 135-142
 - DOI: 10.1016/j.smrv.2006.09.001
 - Key Finding: Noise pollution is a public health concern affecting sleep

---

## 4. Motion Detection Standards

### Optimal Standard
**Minimal movement during sleep (0-5 movements per hour)**

### Scientific Basis
Excessive body movements indicate sleep fragmentation, restlessness, or sleep disorders. Actigraphy studies show correlation between movement frequency and sleep quality.

### Scoring Criteria

| Movement Count (per hour) | Points | Quality |
|----------------------------|--------|---------|
| 0-5 | 100% | Restful |
| 6-15 | 70% | Moderate |
| 16-30 | 40% | Restless |
| >30 | 10% | Very Restless |

### Research References

1. **Ancoli-Israel, S., et al. (2003)**
 - Title: "The role of actigraphy in the study of sleep and circadian rhythms"
 - Journal: Sleep, 26(3), 342-392
 - DOI: 10.1093/sleep/26.3.342
 - Key Finding: Actigraphy is valid method for assessing sleep-wake patterns

2. **Sadeh, A. (2011)**
 - Title: "The role and validity of actigraphy in sleep medicine: an update"
 - Journal: Sleep Medicine Reviews, 15(4), 259-267
 - DOI: 10.1016/j.smrv.2010.10.001
 - Key Finding: Movement patterns correlate with sleep quality

3. **Lichstein, K. L., et al. (2006)**
 - Title: "Actigraphy validation with insomnia"
 - Journal: Sleep, 29(2), 232-239
 - DOI: 10.1093/sleep/29.2.232
 - Key Finding: Increased movement associated with poorer sleep quality

---

## Implementation Notes

### Calculation Method

The ML model calculates sleep quality score using weighted average:

```
Total Score = (Temp_Score × 0.30) + 
 (Humidity_Score × 0.25) + 
 (Sound_Score × 0.25) + 
 (Motion_Score × 0.20)
```

### Data Aggregation

For nightly analysis:
- **Temperature:** Average of all readings during sleep period
- **Humidity:** Average of all readings during sleep period
- **Sound Level:** Average with penalty for peaks >70 dB
- **Motion:** Total count of motion detection events

### Sleep Period Definition

Default analysis period: **10:00 PM to 6:00 AM** (8 hours)

This can be adjusted based on individual sleep patterns.

---

## Limitations and Disclaimers

### System Scope
This system evaluates **environmental factors only**. It does not:
- Diagnose sleep disorders (sleep apnea, insomnia, etc.)
- Measure sleep stages (REM, deep sleep, etc.)
- Assess medical conditions
- Replace professional sleep studies

### Medical Disclaimer
For persistent sleep problems or suspected sleep disorders, consult a qualified healthcare provider or sleep specialist.

### Environmental Assessment
This system provides objective measurement of sleep environment quality based on established research. Individual sleep needs may vary.

---

## Validation

### Research Basis
All standards are derived from:
- Peer-reviewed scientific publications
- International health organization guidelines
- Clinical sleep medicine recommendations
- Environmental health research

### Standard Updates
Standards may be updated as new research emerges. Current version reflects best available evidence as of 2024.

---

## Additional Resources

### Organizations
- **National Sleep Foundation:** https://www.sleepfoundation.org/
- **American Academy of Sleep Medicine:** https://aasm.org/
- **World Health Organization - Noise:** https://www.who.int/health-topics/noise

### Academic Journals
- Sleep Medicine Reviews
- Journal of Sleep Research
- Sleep (Journal of Sleep and Sleep Disorders Research)
- Noise and Health

### Books
- "Sleep Medicine" by Lawrence J. Epstein & Steven Mardon
- "The Sleep Revolution" by Arianna Huffington
- "Why We Sleep" by Matthew Walker

---

## Version History

- **v1.0 (2024-12):** Initial standards documentation
 - Temperature: 18-22°C optimal range
 - Humidity: 40-60% optimal range
 - Sound: <40 dB optimal range
 - Motion: Minimal movement standard
 - Classification: Good (70+) / Poor (<70)

---

## Contact

For questions about standards or research references, please consult the original research papers or contact relevant health organizations.

---

**Note:** This is an educational project. Standards are based on published research but implementation is simplified for educational purposes.


