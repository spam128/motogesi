from django.http import HttpResponse
from django.shortcuts import render


def main(request):
    return HttpResponse("<h1>Hello world!</h1><p>Hello World! 🚀</p>")
