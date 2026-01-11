1. ⭐ Favorites / Bookmarks
```sql
favorites (
  user_id UUID,
  recipe_id UUID,
  created_at TIMESTAMP
)
```
→ Enables:

Favorite recipes page

Like counter

Personalized home feed

---

2. 💬 Comments / Ratings
```sql
recipe_reviews (
  id UUID,
  recipe_id UUID,
  user_id UUID,
  rating INT,
  comment TEXT,
  created_at TIMESTAMP
)
```
→ Enables:

⭐⭐⭐⭐⭐ ratings

Comments

Sorting by rating

---

3. 🏷️ Tag Metadata
```sql
tags (
  id UUID,
  name TEXT,
  description TEXT,
  color TEXT
)
```

---

4. 📈 Analytics
```sql
recipe_views (
  recipe_id UUID,
  user_id UUID NULL,
  viewed_at TIMESTAMP
)
```

---

5. 📝 Drafts / Versioning
```sql
recipe_versions (
  id UUID,
  recipe_id UUID,
  data JSONB,
  created_at TIMESTAMP
)
```