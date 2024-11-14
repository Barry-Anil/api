from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List
from datetime import date

app = FastAPI()

class FabricOrder(BaseModel):
    srno: int
    fabric_status: str
    fabric_status_notes: str
    order_no: str
    order_date: date
    order_since_days: int
    salestrip: str
    payment_type: str
    cc_pass_date: date
    cc_pass_days: int

# In-memory storage for orders
orders_db = []

# GET endpoint to retrieve all fabric orders
@app.get("/orders", response_model=List[FabricOrder])
async def get_fabric_orders():
    return orders_db

# POST endpoint to add multiple fabric orders
@app.post("/orders", status_code=201)
async def create_fabric_orders(new_orders: List[FabricOrder]):
    orders_db.extend(new_orders)
    return {"message": "Orders added successfully"}
