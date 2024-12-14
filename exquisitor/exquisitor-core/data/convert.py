import sqlite3
import pandas as pd

def load_csv_to_sqlite(csv_file, db_file):
    df = pd.read_csv(csv_file)

    conn = sqlite3.connect(db_file)
    cursor = conn.cursor()

    cursor.execute('''
    CREATE TABLE IF NOT EXISTS data (
        row_id INTEGER PRIMARY KEY AUTOINCREMENT,
        anchor TEXT,
        positive TEXT,
        negative TEXT
    )
    ''')

    for index, row in df.iterrows():
        cursor.execute('''
        INSERT INTO data (anchor, positive, negative) VALUES (?, ?, ?)
        ''', (row['anchor'], row['positive'], row['negative']))

    conn.commit()
    conn.close()

load_csv_to_sqlite("training.csv", "training.db")
load_csv_to_sqlite("validation.csv", "validation.db")