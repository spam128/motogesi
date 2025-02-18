from django.urls import path

from dashboard.views import main

app_name = 'dashboard'

urlpatterns = [
    path('', main, name='home'),
]
