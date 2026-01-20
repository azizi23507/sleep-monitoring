import pandas as pd
import joblib
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestRegressor
from sklearn.metrics import mean_absolute_error, root_mean_squared_error, r2_score  
import numpy as np

# ==============================
# 1. Configuration
# ==============================

DATA_PATH = "sleep_raw_50_nights_100_points.csv"
MODEL_PATH = "random_forest_sleep_score.pkl"

# ==============================
# 2. Load CSV data
# ==============================


df = pd.read_csv(DATA_PATH)



df = df.rename(columns={
    "night_id": "nightid",
    "sound_level": "soundlevel",
    "motion_detected": "motiondetected"
})
print(df.columns)


# ==============================
# 3. Feature engineering par nuit
# ==============================


agg_df = df.groupby("nightid").agg(
    avg_temp=("temperature", "mean"),
    temp_variance=("temperature", lambda x: x.max() - x.min()),
    avg_sound=("soundlevel", "mean"),
    sound_peaks=("soundlevel", lambda x: (x > 70).sum()),
    total_motion=("motiondetected", "sum"),
    avg_humidity=("humidity", "mean")
).reset_index()


agg_df["sleep_quality_score"] = 100 \
    - (np.abs(agg_df["avg_temp"] - 18) * 2) \
    - (agg_df["temp_variance"] * 1.5) \
    - (agg_df["avg_sound"] / 5) \
    - (agg_df["sound_peaks"] * 1.0) \
    - (agg_df["total_motion"] * 0.5) \
    - (np.abs(agg_df["avg_humidity"] - 45) * 0.3)


agg_df["sleep_quality_score"] = agg_df["sleep_quality_score"].clip(0, 100)

data = agg_df

if data.empty:
    raise ValueError("No data found in CSV. Cannot train the model.")

print("Number of nights used for training:", len(data))

# ==============================
# 4. Split features / label
# ==============================

X = data[[
    "avg_temp",
    "temp_variance",
    "avg_sound",
    "sound_peaks",
    "total_motion",
    "avg_humidity"
]]

y = data["sleep_quality_score"]

# ==============================
# 5. Train / Test split
# ==============================

X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42
)

# ==============================
# 6. Create and train the model
# ==============================

model = RandomForestRegressor(
    n_estimators=200,
    max_depth=None,
    random_state=42
)

print("Training the model...")
model.fit(X_train, y_train)

# ==============================
# 7. Evaluation
# ==============================

y_pred = model.predict(X_test)

mae = mean_absolute_error(y_test, y_pred)
rmse = root_mean_squared_error(y_test, y_pred) 
r2 = r2_score(y_test, y_pred)

print("\nModel evaluation")
print("MAE  :", round(mae, 2))
print("RMSE :", round(rmse, 2))
print("R²   :", round(r2, 3))

# ==============================
# 8. Save the model
# ==============================

joblib.dump(model, MODEL_PATH)
print("\nModel saved to:", MODEL_PATH)

